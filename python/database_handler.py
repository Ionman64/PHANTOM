import psycopg2

DATABASE_NAME = u"project_analyser"
TABLE_GIT_REPOSITORY = u"git_repository"
TABLE_COMMIT_FREQUENCY = u"commit_frequency"
DATABASE_PW = u"0000"


def drop_database():
    db = get_database("")
    cursor = db.cursor()
    cursor.execute("DROP DATABASE %s", [DATABASE_NAME])
    cursor.close()
    db.commit()


def get_all_projects():
    db = get_database()
    cur = db.cursor()
    cur.execute("SELECT * FROM %s", [TABLE_GIT_REPOSITORY])
    projects = {}
    for row in cur.fetchall():
        projects[row[0]] = row[1]
    return projects


def get_commit_frequency_by_id(repository_id):
    db = get_database()
    cur = db.cursor()
    cur.execute("SELECT * FROM commit_frequency WHERE repository_id=%s",
                [repository_id])
    commit_frequencies = {}
    for row in cur.fetchall():
        commit_frequencies[row[1]] = row[2]
    return commit_frequencies


def get_database(db_name=DATABASE_NAME):
    """ Connects to the database and return the mysql object. Pass an empty string to create the database"""
    return psycopg2.connect(
        host="localhost",
        database=db_name,
        user="postgres",
        password=DATABASE_PW)


if __name__ == "__main__":
    get_all_projects()
