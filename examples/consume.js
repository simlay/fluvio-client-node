

let flv = require('../dist');
// let flv = require('@fluvio/client');


const EventEmitter = require('events').EventEmitter;
const emitter = new EventEmitter();

emitter.on('data', (record) => {

    // console.log("received record",record);
    console.log("received event", record.offset, Buffer.from(record.record).toString());

})

console.log("connecting client to sc");
flv.connect().then(sc => {
    console.log("connect to sc at ", sc.addr());

    sc.replica("test1", 0).then(replica => {

        try {

            /*
            replica.consume(
                emitter.emit.bind(emitter),
                "earliest"
            );
            */

            /*
            // start from absolute offset 5
            replica.consume(
                emitter.emit.bind(emitter),
                5
            );
            */

            /*
            // start from last 2
            replica.consume(
                emitter.emit.bind(emitter),
                {
                    offset: 2,
                    start: 'latest'
                }
            );
            */

            /*
            // start from 2 offset from begining
            replica.consume(
                emitter.emit.bind(emitter),
                {
                    offset: 2,
                    start: 'earliest'
                }
            );
            */

            // start from absolute offset 5
            replica.consume(
                emitter.emit.bind(emitter),
                {
                    offset: 0
                }
            );


        } catch (ex) {
            console.log(ex);
        }
    })
})
    .catch((err) => console.log(err));
