import psycopg2

DATABASE_NAME = u"project_analyser"
DATABASE_PW = u"new"

def get_config_params():
    return_config = {}
    try:
        with open("../.env") as file:
            for line in file:
                columns = line.replace("\n", "").split("=")
                return_config[columns[0]] = columns[1]
        return return_config
    except IOError as e:
        raise Exception("Could not find properties file")
    except IndexError as e:
        raise Exception("Properties file formatted incorrectly")
    except Exception as e:
        raise Exception("Could not read properties file")
    return None

CONFIG = get_config_params();

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


def get_commit_frequency_by_id(db, repository_id):
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
        host=CONFIG["DATABASE_HOST"],
        database=CONFIG["DATABASE_NAME"],
        user=CONFIG["DATABASE_USER"],
        password=CONFIG["DATABASE_PASSWORD"])


if __name__ == "__main__":
    get_all_projects()
