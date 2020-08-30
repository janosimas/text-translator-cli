use clap::{value_t, App, Arg};
use text_translator::{Api, ApiDetect, GoogleV2, InputLanguage, Language};

fn main() {
    let langs = Language::iterator()
        .map(Language::to_language_code)
        .collect::<Vec<&str>>();
    let args = App::new("Text trasnlator cli")
        .arg(Arg::from_usage("-k <key> --key 'API key'").required(true))
        .arg(
            Arg::from_usage("-d --detect 'Detect input language'")
                .conflicts_with("input")
                .conflicts_with("output")
                .required_unless("output")
                .requires("text"),
        )
        .arg(
            Arg::from_usage("-i [input] --input 'Input language'")
                .possible_values(&langs)
                .requires("text"),
        )
        .arg(
            Arg::from_usage("-o <output> --output 'Output language'")
                // Define the list of possible values
                .possible_values(&langs)
                .required_unless("detect")
                .conflicts_with("detect")
                .requires("text"),
        )
        .arg(Arg::from_usage("-t <text> --text 'Input text'").required(true))
        .get_matches();

    let text = args.value_of("text").unwrap();
    println!("text: {}", text);
    let key = args.value_of("key").unwrap();

    let translator = GoogleV2::with_key(key);
    if args.is_present("detect") {
        let language = translator.detect(text.to_string()).unwrap();
        println!("Input language: {:?}", language);
    } else {
        let input_lang = if args.is_present("input") {
            InputLanguage::Defined(value_t!(args, "input", Language).unwrap())
        } else {
            InputLanguage::Automatic
        };

        let output_lang = value_t!(args, "output", Language).unwrap();
        println!("input language: {:?}", input_lang);
        println!("output language: {:?}", output_lang);

        let translation = translator
            .translate(text.to_string(), input_lang, output_lang)
            .unwrap();
        println!("translated text: {}", translation);
    }
}
