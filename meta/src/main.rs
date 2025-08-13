use std::fs;

const INDEX: &str = "This page lists basic information about me and some social media sites I am \
on. If you want to get in touch with me, you are best off sending me an email.";

const CANONICAL: &str = r#"<!-- Custom HTML head -->
        <link rel="canonical" href="https://dlc.name" />"#;

const BIOGRAPHY: &str = "This page details my biography related to software engineering. It \
includes my studies at Brooklyn Collage, my side projects, and my employment history.";

const PROJECTS: &str = "This page lists projects I have worked on related to software. It includes \
a poker project, a cubes project, a Julia Benchmarks project, and cparted.";

const RESUME: &str = "This is my resume. It lists everything I have accomplished related to \
software engineering. It includes my work history and other relevant details.";

fn main() -> Result<(), anyhow::Error> {
    let index_path = "./book/index.html";
    let file = fs::read_to_string(index_path)?;
    let content = file.replace("{{description}}", INDEX);
    fs::write(index_path, content)?;

    let file = fs::read_to_string(index_path)?;
    let content = file.replace("<!-- Custom HTML head -->", CANONICAL);
    fs::write(index_path, content)?;

    let install_path = "./book/biography.html";
    let file = fs::read_to_string(install_path)?;
    let content = file.replace("{{description}}", BIOGRAPHY);
    fs::write(install_path, content)?;

    let install_path = "./book/projects.html";
    let file = fs::read_to_string(install_path)?;
    let content = file.replace("{{description}}", PROJECTS);
    fs::write(install_path, content)?;

    let rules_path = "./book/resume.html";
    let file = fs::read_to_string(rules_path)?;
    let content = file.replace("{{description}}", RESUME);
    fs::write(rules_path, content)?;

    Ok(())
}
