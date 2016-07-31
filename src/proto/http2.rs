
#[macro use] extern crate microstate;

#[macro use] extern crate nom;


use nom::IResult;

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
microstate! {
    ClientStream { Init };

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
microstate! {
    ServerStream { Init };

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
    Http2Parser { Init };
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
