
#[macro use] extern crate microstate;

#[macro use] extern crate nom;


use nom::IResult;
use block_allocator::Allocator;
use std::collections::LinkedList;

#[derive(Copy, Clone)]
pub enum Http2Event {
    Data,
    Headers,
    PushPromise,
    Settings,
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

enum FrameCode {
    Data = 0x0,
    Headers = 0x1,
    Priority = 0x2,
    RstStream = 0x3,
    Settings = 0x4,
    PushPromise = 0x5,
    Ping = 0x6,
    GoAway = 0x7,
    WindowUpdate = 0x8
}

enum ErrorCode {
   NoError = 0x0,
    /* *The associated condition is not as a result of an
     * error.  For example, a GOAWAY might include this code to indicate
     * graceful shutdown of a connection.  */

   ProtocolError = 0x1, 
    /* The endpoint detected an unspecific protocol
     * error.  This error is for use when a more specific error code is
     * not available.  */

   InternalError = 0x2,
   /* The endpoint encountered an unexpected
    * internal error.  */

   FlowControlError = 0x3,
   /*  The endpoint detected that its peer
    *  violated the flow control protocol.*/

   SettingsTimeout = 0x4,
   /*  The endpoint sent a SETTINGS frame, but did
    *  not receive a response in a timely manner.  See Settings
    *  Synchronization (Section 6.5.3). */

   StreamClosed = 0x5,
   /* The endpoint received a frame after a stream
      was half closed.  */

   FrameSizeError = 0x6,
   /* The endpoint received a frame with an
    *  invalid size.  */

   RefusedStream = 0x7, 
   /*  The endpoint refuses the stream prior to
    *  performing any application processing, see Section 8.1.4 for
    *  details.  */

   Cancel = 0x8, 
   /*  Used by the endpoint to indicate that the stream is no
    *  longer needed.  */

   CompressionError = 0x9,
   /*  The endpoint is unable to maintain the
    *  header compression context for the connection.  */

   ConnectError = 0xa,
   /*  The connection established in response to a
    *  CONNECT request (Section 8.3) was reset or abnormally closed.  */

   EnhanceYourCalm = 0xb,
   /* The endpoint detected that its peer is
    *  exhibiting a behavior that might be generating excessive load.  */

   InadequateSecurity = 0xc,
   /*The underlying transport has properties
    *  that do not meet minimum security requirements (see Section 9.2).  */

   HTTP11Required = 0xd,
   /*The endpoint requires that HTTP/1.1 be used
    *  instead of HTTP/2.*/
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
    fn advance(evt : Http2Event) -> Http2State;
}

struct ClientStreamSM {
   state : ClientStreamState 
}

struct ServerStreamSM {
    state : ServerStreamState
}

impl Http2StateMachine for ClientStreamSM {
    fn advance(evt : Http2Event) -> Http2State {
    }
}

impl Http2StateMachine for ServerStreamSM {
    fn advance(evt : Http2Event) -> Http2State {
    }
}

pub struct Stream<'a, SM : Http2StateMachine> {
    in_chain : LinkedList<AppendBuf>,
    out_chain : LinkedList<AppendBuf>,
    stream_state : SM,
    parser_state : ParserState,
    buf_chain : LinkedList<&'a AppendBuf<'a>>
    flow_credit : usize
}

impl<'a, SM : Http2StateMachine> Stream<'a, SM> {
    
    pub fn new_client() -> ProtoResult<Stream<ClientStreamSM>> {
    }

    pub fn new_server() -> ProtoResult<Stream<ServerStreamSM>> {
    }

    pub fn parse(&mut self, buf : AppendBuf) -> ProtoResult<Http2Event> {

    }

    pub fn create(&mut self, frame : Http2Event) -> ProtoResult<(AppendBuf, Http2Event)> {

    }
}

