use crate::tests::fakes::TerminalEvent::*;
use crate::tests::fakes::{create_fake_on_winch, get_interfaces, get_open_sockets, NetworkFrames,
};

use ::insta::assert_snapshot;

use crate::tests::cases::test_utils::{
    opts_ui, test_backend_factory,
};
use pnet::datalink::DataLinkReceiver;

use crate::{start, OsInputOutput};
use crate::tests::fakes::{KeyboardEvents
};
use std::iter;
use ::termion::event::{Event, Key};
use packet_builder::payload::PayloadData;
use packet_builder::*;
use pnet::packet::Packet;
use pnet_base::MacAddr;

use std::collections::HashMap;
use crate::tests::fakes::{
    create_fake_dns_client}; 


fn build_tcp_packet(
    source_ip: &str,
    destination_ip: &str,
    source_port: u16,
    destination_port: u16,
    payload: &'static [u8],
) -> Vec<u8> {
    let mut pkt_buf = [0u8; 1500];
    let pkt = packet_builder!(
         pkt_buf,
         ether({set_destination => MacAddr(0,0,0,0,0,0), set_source => MacAddr(0,0,0,0,0,0)}) /
         ipv4({set_source => ipv4addr!(source_ip), set_destination => ipv4addr!(destination_ip) }) /
         tcp({set_source => source_port, set_destination => destination_port }) /
         payload(payload)
    );
    pkt.packet().to_vec()
}

pub fn dump_sleep_and_quit_events(sleep_num: usize) -> Box<KeyboardEvents> {
    let mut events: Vec<Option<Event>> = iter::repeat(None).take(sleep_num).collect();
    //events.insert(0,Some(Event::Key(Key::Char('d'))));
    events.push(Some(Event::Key(Key::Ctrl('c'))));
    Box::new(KeyboardEvents::new(events))
}

fn os_input_output_dump_factory(
    network_frames: Vec<Box<dyn DataLinkReceiver>>,
    sleep_num: usize,
) -> OsInputOutput {
    let on_winch = create_fake_on_winch(false);
    let cleanup = Box::new(|| {});

    let write_to_stdout: Box<dyn FnMut(String) + Send> = Box::new({ move |_output: String| {} });
    
    OsInputOutput {
        network_interfaces: get_interfaces(),
        network_frames,
        get_open_sockets,
        keyboard_events: dump_sleep_and_quit_events(sleep_num),
        dns_client:create_fake_dns_client(HashMap::new()),
        on_winch,
        cleanup,
        write_to_stdout,
    }
}

#[test]
fn one_packet_of_traffic_with_dump() {
    let network_frames = vec![NetworkFrames::new(vec![Some(build_tcp_packet(
        "10.0.0.2",
        "1.1.1.1",
        443,
        12345,
        b"I am a fake tcp packet",
    ))]) as Box<dyn DataLinkReceiver>];
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let os_input = os_input_output_dump_factory(network_frames, 2);
    let opts = opts_ui();
    start(backend, os_input, opts);
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, Flush, Draw, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}
