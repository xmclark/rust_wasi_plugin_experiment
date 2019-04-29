const exec = require('child_process').exec;

const { series } = require('gulp');

function build_plugin(cb) {
    // body omitted
    exec('cargo +nightly build --package wasm-plugin-foo --target wasm32-unknown-wasi --release', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
}

function build_runner_dev(cb) {
    // body omitted
    exec('cargo build --package wasm-plugin-runner --no-default-features --features dev', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
}

function build_runner_prod(cb) {
    // body omitted
    exec('cargo build --package wasm-plugin-runner --no-default-features --features prod', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
}

function execute_runner_dev(cb) {
    // body omitted
    exec('cargo run --package wasm-plugin-runner --bin wasm-plugin-runner --no-default-features --features dev', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
}

exports.run = execute_runner_dev;
exports.plugin = build_plugin;
exports.build = series(build_plugin, build_runner_prod);
exports.default = series(build_plugin, build_runner_dev);
