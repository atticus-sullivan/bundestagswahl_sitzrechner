"""
Representation of a party
"""
class Partei:
    parteien = {}

    def __init__(self, name):
        self.name = name
        self.direktmandate = 0
        self.ueberHuerde = False
        self.partei_in_land = {}
        self.zweitstimmen = 0

    def update_zweitstimmen(self, zweitstimmen):
        self.zweitstimmen += zweitstimmen

    """
    Checks if party already exists, creates it if not
    """
    @classmethod
    def get(cls, name):
        if str(name) not in cls.parteien:
            cls.parteien[str(name)] = Partei(str(name))
        return cls.parteien[str(name)]

    def __repr__(self):
        return str(self.name)

    def add(self, partei_in_land):
        self.partei_in_land.update({partei_in_land.land.name: partei_in_land})
