import os
class DataHandler():
    UNKNOWN = 0
    DATETIME = 1
    STRING = 2
    NUMBER = 3
    class Units():
        UNKNOWN = 0
        PICOSECOND = 101
        NANOSECOND = 102
        MICROSECOND = 103
        MILLISECOND = 104
        SECOND = 105
        MINUTE = 106
        HOUR = 107
        DAY = 108
        WEEK = 109
        MONTH = 110
        YEAR = 111
        LEAPYEAR = 112
        DECADE = 113
        CENTURY = 114
        MILLENNIUM = 115
    def __init__(self, file_path, cell_format=[], has_headers=True):
        self.rows = []
        self.headers = []
        self.count = 0
        if not os.path.exists(file_path):
            raise Exception("File not found: %s" % file_path)
        with open(file_path, "r") as file:
            for line in file:
                rowStringArray = line.replace("\n", "").split(",")
                while len(cell_format) < len(rowStringArray):
                    cell_format.append(DataHandler.UNKNOWN)
                row_object = Row()
                i = 0
                for cell_string in rowStringArray:
                    cell_string = cell_string.replace("\"", "")
                    if cell_format[i] == DataHandler.NUMBER:
                        row_object.add_cell(Cell(int(float(cell_string)), cell_format[i]))
                    else:
                        row_object.add_cell(Cell(cell_string, cell_format[i]))
                i = i + 1
                if has_headers and self.count == 0:
                    self.headers.append(row_object)
                else:
                    self.rows.append(row_object)
                self.count += 1

    def get_headers(self):
        return self.headers

    def get_rows(self):
        return self.rows

    def get_row(self, rowNum):
        return self.rows[rowNum]

    def num_rows(self):
        return self.count

    def unit_to_string(self, unit):
            if unit == DataHandler.Units.SECOND:
                return "Second"
            if unit == DataHandler.Units.MINUTE:
                return "Minute"
            if unit == DataHandler.Units.HOUR:
                return "Hour"
            if unit == DataHandler.Units.DAY:
                return "Day"
            if unit == DataHandler.Units.WEEK:
                return "Week"
            if unit == DataHandler.Units.YEAR:
                return "Year"
            if unit == DataHandler.Units.DECADE:
                return "Decade"

    def get_number_of_seconds(self, unit):
        if unit == DataHandler.Units.SECOND:
            return 1
        if unit == DataHandler.Units.MINUTE:
            return 60
        if unit == DataHandler.Units.HOUR:
            return 3600
        if unit == DataHandler.Units.DAY:
            return 86400
        if unit == DataHandler.Units.WEEK:
            return 604800
        if unit == DataHandler.Units.YEAR:
            return 31536000
        if unit == DataHandler.Units.DECADE:
            return 315576000    

class Row:
    def __init__(self):
        self.cells = []
    
    def get_cell(self, cellNum):
        return self.cells[cellNum]

    def add_cell(self, cell):
        self.cells.append(cell)

    def get_cells(self):
        return self.cells

class Cell:
    def __init__(self, value, datatype):
        self.value = value
        self.datatype = datatype

    def get_value(self):
        return self.value

    def get_datatype(self):
        return self.datatype