from partei import Partei
from partei_in_land import Partei_in_Land

class Land:
    def __init__(self, name, id):
        self.name = name
        self.id = id
        self.ewz = None
        self.sitzkontingent = None
        self.parteien = {p.name: Partei_in_Land(p.name, self) for p in Partei.parteien.values()}

    def __str__(self):
        return "{[Land] name:" + str(self.name) + ", id:" + str(self.id) + ", ewz:" + str(self.ewz) + ", sitzkontingent:" + str(self.sitzkontingent) + ", parteien:" + str(self.parteien) + "}"
    def __repr__(self):
        return str(self)

    @property
    def name(self):
        return self._name
    @name.setter
    def name(self, name):
        if isinstance(name, str):
            self._name = name
        else:
            raise AttributeError("name has to be a string, but was", type(name))
    @property
    def id(self):
        return self._id
    @id.setter
    def id(self, id):
        if isinstance(id, str):
            self._id = id
        else:
            raise AttributeError("id has to be a string, but was", type(id))
    @property
    def ewz(self):
        return self._ewz
    @ewz.setter
    def ewz(self, ewz):
        if isinstance(ewz, int) or ewz is None:
            self._ewz = ewz
        else:
            raise AttributeError("ewz has to be an int, but was", type(ewz))
    @property
    def sitzkontingent(self):
        return self._sitzkontingent
    @sitzkontingent.setter
    def sitzkontingent(self, sitzkontingent):
        if isinstance(sitzkontingent, int) or sitzkontingent is None:
            self._sitzkontingent = sitzkontingent
        else:
            raise AttributeError("sitzkontingent has to be an int, but was", type(sitzkontingent))

    @property
    def parteien(self):
        return self._parteien
    @parteien.setter
    def parteien(self, parteien):
        if isinstance(parteien, dict):
            self._parteien = parteien
        else:
            raise AttributeError("parteien has to be a dict, but was", type(parteien))

    def put_partei(self, name, erststimmen, zweitstimmen):
        self.parteien[name].update_stimmen(erststimmen, zweitstimmen)
