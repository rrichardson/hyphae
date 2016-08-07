
use tokio::rt::{Dispatch, NewDispatch, Server, Reactor, EventLoop};
use tokio::io::{Transport};
use tokio::rt::server;
use std::collections::{HashMap, LinkedList};
use std::vec::Vec;
use block_alloc_appendbuf::AppendBuf;
use http2_proto::Http2Proto;

struct Http2Connection<'a> {
    machine : Http2Proto;
    conn : TcpStream,
    streams : Vec<Stream<'a>>,
    alloc : Allocator<'a>,
    flow_credit : usize,
    on_new_conn   : Option<EventCallback>,
    on_new_stream : Option<EventCallback>,
    on_data_frame      : Option<DataCallback>,
    on_headers_complete : Option<EventCallback>,
    on_stream_close : Option<EventCallback>,
    on_conn_close : Option<EventCallback>
}

struct Http2Frame {
    size : usize,
    options : HashMap<&[u8], &[u8]>,
    data : AppendBuf
}

impl Transport for Http2Connection {
    type In = Http2Frame;
    type Out = Http2Frame;
    fn read(&mut self) -> io::Result<Option<Http2Frame>> {
        if self.conn.is_readable() {
            let buf = self.in_chain.back_mut();
            let len = conn.read(buf)
            let frame = machine.parse_frame(buf); // <-- lots going on right here :)
            return Ok(frame)
        }
    }

    fn write(&mut self, frame : Http2Frame) -> io::Result<Option<()>> {
        if (self.out_chain.len() > 0) {
            self.inner_flush();
        }
        let len = frame.data().len();
        let sent = self.conn.write(frame.data());
        if sent < len {
            let buf = frame.data().clone();
            buf.advance(sent);
            self.out_chain.push_back(buf);
        }
    }

    fn flush(&mut self) -> io::Result<Option<()>> {
        if (self.out_chain.len() > 0) {
            self.inner_flush();
        }
    }

    fn inner_flush(&mut self) -> io::Result<Option<()>> {
        //TBD
    }
}

impl Dispatch for Http2Connection {
    fn tick(&mut self) -> io::Result<Self::Item> {
        if let frame = self.read() {
            self.sm_update(frame);
            match frame.frame_type {
                NewStream => self.on_new_stream(StreamEvent(frame));
                DataFrame => self.on_data_frame(DataEvent(frame));
                ...
            }
        }
    }
}

impl NewDispatch for Http2Connection {
    type Item = Http2Connection;
  
    fn new_dispatch(&self, stream: TcpStream) -> io::Result<Self::Item> {
        Http2Connection {
            machine : Http2Proto::new();
            conn : stream,
            in_chain : LinkedList<AppendBuf>::new(),
            out_chain : LinkedList<AppendBuf>::new(),
            alloc : g_allocator, 
            on_new_conn   : None,
            on_new_stream : None,
            on_data_frame      : None,
            on_headers_complete : None,
            on_stream_close : None,
            on_conn_close : None
        }

    }
 
}

fn main() {
    let el = EventLoop::new();
    let reactor = Reactor::new();

    server::listen("127.0.0.1:80", reactor);
    
}
