
#[macro use] extern crate microstate;

#[macro use] extern crate nom;


use nom::IResult;
use block_allocator::Allocator;
use std::collections::LinkedList;

pub enum Http2Event {
    Headers,
    PushPromise,
    Settings,
    Data,
    Continuation,
    Preface,
    EndStream,
    Idle
}

enum Http2State {
        Init
        Idle,
        HalfOpenLocal,
        HalfOpenRemote,
        ReserveRemote,
        ReserveLocal,
        Open,
        HalfClosedLocal,
        HalfClosedRemote,
        Closed
}

enum ProtoError {
    ParseIncomplete<u32>
}

type ProtoResult<T> = std::Result<T, ProtoError>;

///
/// State machines and parsers for the http2 protocol
///

//lifted directly from RFC7540 5.1 Figure 2
//This represents the client stream, so certain
//state transitions are not possible, namely 
//the sending of push_promises.  In addition
//the push_promise resulting state differs from
//diagram because that event results in a new
//stream being created, and the new stream begins
//in that state

//Added additional states to reflect a possible need to send 
//a preface for the very first connection, for all subsequently
//created streams, the constructor of the stream will issue the
//idle() event
microstate_ext! {
    ClientStreamState { Init, Http2State };

    states {
        Init
        Idle,
        HalfOpenLocal,
        ReserveRemote,
        Open,
        HalfClosedLocal,
        HalfClosedRemote,
        Closed
    }
    
    preface_send {
        Init => HalfOpenLocal
    }

    idle {
        Init => Idle
    }

    settings_recv {
        HalfOpenLocal => Idle
    }

    preface_recv {
        Init => Idle
    }

    preface_send {
        Init => Idle
    }

    header_send {
        Idle => Open
    }

    header_recv {
        Idle => Open
    }

    push_promise_recv {
        Idle => ReservedRemote
    }

    end_stream_recv {
        Open => HalfClosedRemote,
        HalfClosedRemote => Closed
    }

    end_stream_send {
        Open => HalfClosedLocal,
        HalfClosedLocal => Closed
    }

    reset_recv {
        Open => Closed
        HalfClosedLocal => Closed
        HalfClosedRemote => Closed
        ReservedRemote => Closed
    }
}

//lifted directly from RFC7540 5.1 Figure 2
//Added additional states to reflect a possible need to send 
//a preface for the very first connection, for all subsequently
//created streams, the constructor of the stream will issue the
//idle() event
microstate_ext! {
    ServerStreamState { Init, Http2State };

    states {
        Init,
        HalfOpenRemote,
        Idle,
        ReservedLocal,
        Open,
        HalfClosedLocal,
        HalfClosedRemote,
        Closed
    }

    idle {
        Init => Idle
    }

    preface_recv {
        Init => HalfOpenRemote
    }

    settings_send {
        HalfOpenRemote => Idle
    }

    header_send {
        Idle => Open
    }

    header_recv {
        Idle => Open
    }

    push_promise_send {
        Idle => ReservedLocal
    }

    end_stream_recv {
        Open => HalfClosedRemote,
        HalfClosedRemote => Closed
    }

    end_stream_send {
        Open => HalfClosedLocal,
        HalfClosedLocal => Closed
    }

    reset_recv {
        Open => Closed
        HalfClosedLocal => Closed
        HalfClosedRemote => Closed
        ReservedLocal => Closed
    }
}


microstate! {
    ParserState { Init };
    states {
        Init, AwaitFrameHeader, AwaitHeader, AwaitFrame, Closed
    }

    preface {
        Init => AwaitHeader
    }

    header {
       AwaitHeader => AwaitFrame 
    }

    frame_recv {
        AwaitFrame => AwaitFrame
    }

    frame_send {
        AwaitFrame => AwaitFrame
        AwaitHeader => AwaitHeader
    }

    close {
        AwaitFrame => Closed
    }
}

trait Http2StateMachine {
    fn advance(evt : Http2Event) -> Http2State {
    }
}

struct ClientStreamSM {
   state : ClientStreamState 
}

struct ServerStreamSM {
    state : ServerStreamState
}

impl Http2StateMachine ClientStreamSM {
    fn advance(evt : Http2Event) -> Http2State {
    }
}

impl Http2StateMachine for ServerStreamSM {
    fn advance(evt : Http2Event) -> Http2State {
    }
}

struct Stream<'a, SM : Http2StateMachine> {
    stream_state : SM,
    parser_state : ParserState,
    buf_chain : LinkedList<&'a [u8]>
}

impl<'a, SM : Http2StateMachine> Stream<'a, SM> {
    
    pub fn new_client() -> ProtoResult<Stream<ClientStreamSM> {
    }

    pub fn new_server() -> Stream<ClientStreamSM> {
    }

    pub fn parse(&mut self, buf : AppendBuf) -> ProtoResult<Http2Event> {

    }

    pub fn create(&mut self, frame : Http2Event) -> ProtoResult<(AppendBuf, Http2Event)> {

    }
}

