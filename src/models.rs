use super::schema::github_projects;

#[derive(Queryable)]
pub struct GitHubProject {
    pub id: i64,
    pub url: String,
}


#[derive(Insertable)]
#[table_name="github_projects"]
pub struct NewGitHubProject {
    pub url: String
}