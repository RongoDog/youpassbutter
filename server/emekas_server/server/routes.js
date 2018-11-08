exports.moveUp = function(req, res) {
    console.log("Up: Moved");
    res.sendStatus(200);
};

exports.moveDown = function(req, res) {
    // kafka.brokers is a Brokers instance, list() returns a list of Broker instances
    metrics.increment('memory.loginCount');
    console.log("metrics login incremented");
    res.sendStatus(200);
};