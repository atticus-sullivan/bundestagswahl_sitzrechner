from partei import Partei
from partei_in_land import Partei_in_Land

class Land:
    def __init__(self, name, id):
        self.name = name
        self.id = id
        self.ewz = 0
        self.sitzkontingent = 0
        self.parteien = {p.name: Partei_in_Land(p.name, self) for p in Partei.parteien.values()}

    def put_partei(self, name, erststimmen, zweitstimmen):
        self.parteien[name].update_stimmen(erststimmen, zweitstimmen)

    def __str__(self):
        return "{name:" + self.name + ", id:" + self.id + ", ewz:" + str(self.ewz) + ", sitzkontingent:"  + str(self.sitzkontingent) # + ", parteien" + str(self.parteien) + "}"

    def __repr__(self):
        return self.__str__()
