const childProcess = require('child_process');

function spawnBinary(cmd, event, callback) {
	let args = [];
	args.push(event.httpMethod);
	args.push(event.path);
	addToArgs(event.queryStringParameters, args);
	addToArgs(event.pathParameters, args);
	let proc = childProcess.spawn(`./${cmd}`, args, { cwd: 'bin' });
	let stdout = '';
	proc.stdout.on('data', data => {
		stdout += data.toString();
	});
	if (!!event.body) {
		proc.stdin.write(event.body.toString());
		proc.stdin.end();
	}
	proc.on('close', code => {
		const response = {
			statusCode: code + 200,
			headers: {
				'Access-Control-Allow-Origin': '*', // Required for CORS support to work
			},
			body: stdout,
		};
		callback(null, response);
	});
}

function addToArgs(obj, args) {
	if (!obj) {
		return;
	}
	for (let key in obj) {
		if (!obj.hasOwnProperty(key)) {
			continue;
		}
		args.push(`--${key}`);
		args.push(obj[key]);
	}
}

module.exports = {
	spawnBinary,
};
