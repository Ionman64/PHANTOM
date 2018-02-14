def get_db_connection_string(path="../.env"):
    env_map = {}
    with open(path) as file:
        for line in file.readlines():
            line = line.split('=')
            assert (len(line) == 2)
            env_map[line[0]] = line[1]
    return env_map.get("DATABASE_URL")