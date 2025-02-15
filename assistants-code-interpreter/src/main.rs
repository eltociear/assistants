// export $(cat .env | xargs)
// cargo run --package assistants-code-interpreter --bin assistants-code-interpreter
// 1.2 times 6 power 2.3

// docker run --rm code-interpreter python -c "print(1+1)"

// TODO: copy paste https://github.com/KillianLucas/open-interpreter into a safe server-side environment that generate and execute code

use assistants_core::function_calling::generate_function_call;
use assistants_core::function_calling::Function;
use assistants_core::function_calling::FunctionCallInput;
use assistants_core::function_calling::ModelConfig;
use assistants_core::function_calling::Parameter;
use assistants_core::function_calling::Property;
use bollard::container::LogOutput;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::exec::CreateExecOptions;
use bollard::exec::StartExecResults;
use bollard::models::HostConfig;
use bollard::Docker;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get user input
    print!("Enter your question: ");
    io::stdout().flush()?;
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let result = interpreter(user_input).await?;
    println!("Result: {}", result);
    Ok(())
}

async fn interpreter(user_input: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("Generating Python code...");

    let build_prompt = |user_input: &str| {
        format!("
You are an Assistant that generate Python code from user input to do complex computations. We execute the code you will generate and return the result to the user.
Given this user input: {}, generate Python code that we will execute and return the result to the user.

Rules:
- You can use these libraries: pandas numpy matplotlib scipy
- Only return Python code. If you return anything else it will trigger a chain reaction that will destroy the universe. All humans will die and you will disappear from existence.
- Make sure to use the right numbers e.g. with the user ask for the square root of 2, you should return math.sqrt(2) and not math.sqrt(pd.DataFrame({{'A': [1, 2, 3], 'B': [4, 5, 6]}})).
- Do not use any library if it's simple math (e.g. no need to use pandas to compute the square root of 2)

A few examples:

The user input is: compute the square root of pi
The Python code is:
import math
print(\"The square root of pi is: \" + str(math.sqrt(math.pi)))

The user input is: raising $27M at a $300M valuation how much dilution will the founders face if they raise a $58M Series A at a $2B valuation?
The Python code is:
raise_amount = 27_000_000
post_money_valuation = 300_000_000

series_a_raise_amount = 58_000_000
series_a_post_money_valuation = 2_000_000_000

founders_dilution = (raise_amount / post_money_valuation) * 100
series_a_dilution = (series_a_raise_amount / series_a_post_money_valuation) * 100

print(\"Founders dilution: \" + str(founders_dilution) + \"%\")

So generate the Python code that we will execute that can help the user with this question: {}
        ", user_input, user_input)
    };

    // Generate Python code
    let function_call_input = FunctionCallInput {
        function: Function {
            user_id: "user1".to_string(),
            name: "exec".to_string(),
            description: "A function that executes Python code".to_string(),
            parameters: Parameter {
                r#type: String::from("object"),
                required: Some(vec![String::from("code")]),
                properties: {
                    let mut map = HashMap::new();
                    map.insert(
                        String::from("code"),
                        Property {
                            r#type: String::from("string"),
                            description: Some(String::from("The Python code to execute")),
                            r#enum: None,
                        },
                    );
                    Some(map)
                },
            },
        },
        user_context: build_prompt(&user_input),
        model_config: ModelConfig {
            model_name: String::from("open-source/llama-2-70b-chat"),
            model_url: Some("https://api.perplexity.ai/chat/completions".to_string()),
            user_prompt: user_input.clone(), // not used imho
            temperature: Some(0.0),
            max_tokens_to_sample: 200,
            stop_sequences: None,
            top_p: Some(1.0),
            top_k: None,
            metadata: None,
        },
    };

    let function_result = generate_function_call(function_call_input).await?;
    println!("Function result: {:?}", function_result);
    let python_code = function_result.parameters.unwrap();
    let python_code = python_code.get("code").unwrap();

    println!("Python code generated {:?}", python_code);

    // Connect to Docker
    let docker = Docker::connect_with_local_defaults()?;

    println!("Creating Docker container...");

    // Create Docker container
    let config = Config {
        image: Some("code-interpreter"),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            ..Default::default()
        }),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        open_stdin: Some(true),
        tty: Some(true),
        ..Default::default()
    };
    let options = CreateContainerOptions {
        name: "my-python-container",
    };
    let container = docker.create_container(Some(options), config).await?;

    println!("Starting Docker container...");

    // Start Docker container
    docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await?;

    // non interactive
    let exec = docker
        .create_exec(
            &container.id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["python", "-c", &python_code]),
                ..Default::default()
            },
        )
        .await?
        .id;
    let mut exec_stream_result = docker.start_exec(&exec, None);

    let mut output = String::new();
    while let Some(Ok(msg)) = exec_stream_result.next().await {
        match msg {
            StartExecResults::Attached { log, .. } => match log {
                LogOutput::StdOut { message } => {
                    output.push_str(&String::from_utf8(message.to_vec()).unwrap());
                }
                LogOutput::StdErr { message } => {
                    output.push_str(&String::from_utf8(message.to_vec()).unwrap());
                }
                _ => (),
            },
            _ => (),
        }
    }

    // remove container
    docker
        .remove_container(
            &container.id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    #[ignore]
    async fn test_interpreter() {
        dotenv().ok();

        let inputs = vec![
            "Compute the factorial of 10",
            "Calculate the standard deviation of the numbers 1, 2, 3, 4, 5",
            "Find the roots of the equation x^2 - 3x + 2 = 0",
            "Calculate the area under the curve y = x^2 from x = 0 to x = 2",
            "Compute the integral of x^2 from 0 to 1",
            "Calculate the determinant of the matrix [[1, 2], [3, 4]]",
            "Solve the system of equations: 2x + 3y = 7 and x - y = 1",
            "Compute the eigenvalues of the matrix [[1, 2], [3, 4]]",
            "Calculate the dot product of the vectors [1, 2, 3] and [4, 5, 6]",
            "Compute the cross product of the vectors [1, 2, 3] and [4, 5, 6]",
            "Calculate the Fourier transform of the function f(t) = t^2 for t from -1 to 1",
            "Compute the inverse of the matrix [[1, 2, 3], [4, 5, 6], [7, 8, 9]]",
            "Solve the differential equation dy/dx = y^2 with initial condition y(0) = 1",
            "Calculate the double integral of x*y over the rectangle [0, 1] x [0, 1]",
            "Compute the Laplace transform of the function f(t) = e^(-t) * sin(t)",
            "Find the shortest path in the graph with edges {(A, B, 1), (B, C, 2), (A, C, 3)}",
            "Calculate the convolution of the functions f(t) = t and g(t) = t^2",
            "Compute the eigenvalues and eigenvectors of the matrix [[1, 2, 3], [4, 5, 6], [7, 8, 9]]",
            "Solve the system of linear equations: 2x + 3y - z = 1, x - y + 2z = 3, 3x + y - z = 2",
            "Calculate the triple integral of x*y*z over the cube [0, 1] x [0, 1] x [0, 1]",
        ];

        for input in inputs {
            let result = interpreter(input.to_string()).await;
            assert!(
                result.is_ok(),
                "Failed on input: {} error: {:?}",
                input,
                result
            );
            let result_string = result.unwrap();
            assert!(
                result_string.len() > 0,
                "Failed on input: {}. Result: {}",
                input,
                result_string
            );
        }
    }
}
