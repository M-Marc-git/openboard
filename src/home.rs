
pub fn generate_home() -> String {
    let projects = std::fs::read_to_string("db/project.db")
        .unwrap();
    let projects: Vec<_> = projects.split('\n').collect();
    let mut project_board = String::from("");
    for project in projects {
        project_board += 
            format!("<p><a href=\"{project}\">{project}</a></p>").as_str();
    }
    let base_page = std::fs::read_to_string("page/index.html").unwrap();
    let data = base_page.replace("{projects_list}", &project_board);
    let length = data.len();

    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{data}")
}
