import psycopg2

def get_db_user():
	return dev_config.DB_USER

def get_db_password():
	return dev_config.DB_PASSWORD

def drop_database():
	db = get_database("")
	cursor = db.cursor()
	cursor.execute("DROP DATABASE %s" % get_database_name())
	cursor.close()
	db.commit()

def get_all_projects():
    db = get_database()
    cur = db.cursor()
    cur.execute("SELECT * FROM github_projects")
    projects = {}
    for row in cur.fetchall():
        projects[row[0]] = row[1]
    return projects


def get_database_name():
	""" Return the name of the database"""
	return "project_analyser"

def get_database(db_name = get_database_name()):	
	""" Connects to the database and return the mysql object. Pass an empty string to create the database"""
	return psycopg2.connect(host="localhost", database=db_name, user="postgres", password="new")

if __name__ == "__main__":
    get_all_projects()
