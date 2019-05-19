Pebble.addEventListener('ready', function () {

    Pebble.addEventListener('appmessage', function (d) {
        console.log("Got AppMessage", JSON.stringify(d.payload))
    })

    var dict = {}
    dict['App_ExampleKey'] = 'Hello from JavaScript!'
    Pebble.sendAppMessage(dict, function() {
        console.log('Success!')
    }, function(d, e) {
        console.log('Error', d, e)
    })
})