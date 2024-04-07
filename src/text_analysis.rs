// use rust_bert::pipelines::generation::{BartGenerator, GenerateConfig, LanguageGenerator};
// use rust_bert::resources::{download_resource, Resource};
// use rust_bert::Config;
// use tch::Device;
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // // Define the paths for the model and the vocab files
//     // let model_resource = Resource::Remote {
//     //     url: "https://cdn.huggingface.co/facebook/bart-large-cnn/model.ot",
//     //     ..Default::default()
//     // };
//     // let vocab_resource = Resource::Remote {
//     //     url: "https://cdn.huggingface.co/facebook/bart-large-cnn/vocab.json",
//     //     ..Default::default()
//     // };
//     // let merges_resource = Resource::Remote {
//     //     url: "https://cdn.huggingface.co/facebook/bart-large-cnn/merges.txt",
//     //     ..Default::default()
//     // };
//     //
//     // // Download the files
//     // let model_path = download_resource(&model_resource).await?;
//     // let vocab_path = download_resource(&vocab_resource).await?;
//     // let merges_path = download_resource(&merges_resource).await?;
//     //
//     // // Load the model
//     // let generate_config = GenerateConfig {
//     //     model_resource: Resource::Local { local_path: model_path },
//     //     vocab_resource: Resource::Local { local_path: vocab_path },
//     //     merges_resource: Resource::Local { local_path: merges_path },
//     //     config_resource: Resource::Remote {
//     //         url: "https://cdn.huggingface.co/facebook/bart-large-cnn/config.json",
//     //         ..Default::default()
//     //     },
//     //     max_length: 142,
//     //     device: Device::Cpu,
//     //     ..Default::default()
//     // };
//     // let summarizer = BartGenerator::new(generate_config)?;
//     //
//     // // Generate a summary
//     // let input_context = "The tower is 324 metres (1,063 ft) tall, about the same height as an 81-storey building, and the tallest structure in Paris. Its base is square, measuring 125 metres (410 ft) on each side. During its construction, the Eiffel Tower surpassed the Washington Monument to become the tallest man-made structure in the world, a title it held for 41 years until the Chrysler Building in New York City was finished in 1930. It was the first structure to reach a height of 300 metres. Due to the addition of a broadcasting aerial at the top of the tower in 1957, it is now taller than the Chrysler Building by 5.2 metres (17 ft). Excluding transmitters, the Eiffel Tower is the second tallest free-standing structure in France after the Millau Viaduct.";
//     // let output = summarizer.generate(Some(vec![input_context]), None);
//     //
//     // // Print the summary
//     // for sentence in output {
//     //     println!("{}", sentence);
//     // }
//
//     Ok(())
// }