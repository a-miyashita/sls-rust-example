'use strict';

const spawnBinary = require("./lib.js").spawnBinary;

module.exports.hello_world = (event, context, callback) => {
	spawnBinary('hello_world', event, callback);
};

module.exports.todos_dynamodb = (event, context, callback) => {
	spawnBinary('todos_dynamodb', event, callback);
};

