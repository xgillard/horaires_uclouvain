extern crate clap;
#[macro_use]
extern crate dotenv_codegen;
extern crate webbrowser;

/// The heart of the program: opens up a web page displaying your roster for the current year.
fn main() {
    let args = arguments();
    let url  = build_url(args.value_of("COURSES").unwrap_or(dotenv!("horaires_courses")));
    
    webbrowser::open(&url)
        .expect(&format!("Could not open the default browser at url : {}", &url));
}

/// Handles the command line arguments
fn arguments<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("horaire")
        .version("1.0")
        .author("Xavier Gillard <xavier.gillard@uclouvain.be>")
        .about("Vous redirige vers la page ADE avec *votre* horaire.")
        .arg(clap::Arg::with_name("COURSES")
             .help("La liste de vos cours séparés par une virgule. (Env.Var $COURSES p/ defaut)")
             .required(false))
        .get_matches()
}

/// Builds an URL to display the roster with the given courses
fn build_url(courses: &str) -> String {
    let protocol = dotenv!("horaires_protocol");
    let hostname = dotenv!("horaires_hostname");
    let path     = dotenv!("horaires_path");
    let login    = dotenv!("horaires_login");   
    let password = dotenv!("horaires_password");
    let code     = courses.replace(" ", "");
    
    format!(
        "{proto}://{host}{path}?login={login}&password={pass}&code={code}",
        proto = protocol,
        host  = hostname,
        path  = path,
        login = login,
        pass  = password,
        code  = code
    )
}
