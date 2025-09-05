const { loadPolicy } = require("@open-policy-agent/opa-wasm");
const fs = require("fs");

const policyWasm = fs.readFileSync("/home/andreatp/workspace/opa-java-wasm/core/target/compiled-policies/issue107/policy.wasm");

async function evaluate(input) {
    console.log("input: " + input);
    const policy = await loadPolicy(policyWasm);

    resultSet = policy.evaluate(input);
    if (resultSet == null) {
        console.error("evaluation error");
    } else if (resultSet.length == 0) {
        console.log("undefined");
    } else {
        console.log("result: " + JSON.stringify(resultSet));
    }
}

evaluate({"role": "admin","name": "Doe"});
evaluate({"role": "admin","name": "Štěpán"});
evaluate({"role": "admin","name": "\\"});
