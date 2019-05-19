Pebble.addEventListener('ready', function () {
    var dict = {
        'App_ExampleKey': 'Hello from JavaScript!'
    }
    Pebble.sendAppMessage(dict)
})