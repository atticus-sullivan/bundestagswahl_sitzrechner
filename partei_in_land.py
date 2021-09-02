from partei import Partei
import land as Land

class Partei_in_Land:
    def __init__(self, name, land):
        self.name = name
        self.land = land
        self.partei = Partei.get(name)
        self.partei.add(self)
        
        self.direktmandate = 0
        self.erststimmen = None #TODO not nessacary
        self.zweitstimmen = None

        self.sitzkontingentVerteilung = None
        self.mindestsitzzahl = None

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
    def land(self):
        return self._land
    @land.setter
    def land(self, land):
        if isinstance(land, Land.Land):
            self._land = land
        else:
            raise AttributeError("land has to be a Land, but was", type(land))
    @property
    def partei(self):
        return self._partei
    @partei.setter
    def partei(self, partei):
        if isinstance(partei, Partei):
            self._partei = partei
        else:
            raise AttributeError("partei has to be a Partei, but was", type(partei))
    @property
    def direktmandate(self):
        return self._direktmandate
    @direktmandate.setter
    def direktmandate(self, direktmandate):
        if isinstance(direktmandate, int):
            self._direktmandate = direktmandate
        else:
            raise AttributeError("direktmandate has to be an int, but was", type(direktmandate))
    @property
    def zweitstimmen(self):
        return self._zweitstimmen
    @zweitstimmen.setter
    def zweitstimmen(self, zweitstimmen):
        if isinstance(zweitstimmen, int) or zweitstimmen is None:
            self._zweitstimmen = zweitstimmen
        else:
            raise AttributeError("zweitstimmen has to be an int, but was", type(zweitstimmen))
    @property
    def sitzkontingentVerteilung(self):
        return self._sitzkontingentVerteilung
    @sitzkontingentVerteilung.setter
    def sitzkontingentVerteilung(self, sitzkontingentVerteilung):
        if isinstance(sitzkontingentVerteilung, int) or sitzkontingentVerteilung is None:
            self._sitzkontingentVerteilung = sitzkontingentVerteilung
        else:
            raise AttributeError("sitzkontingentVerteilung has to be an int, but was", type(sitzkontingentVerteilung))
    @property
    def mindestsitzzahl(self):
        return self._mindestsitzzahl
    @mindestsitzzahl.setter
    def mindestsitzzahl(self, mindestsitzzahl):
        if isinstance(mindestsitzzahl, int) or mindestsitzzahl is None:
            self._mindestsitzzahl = mindestsitzzahl
        else:
            raise AttributeError("mindestsitzzahl has to be an int, but was", type(mindestsitzzahl))

    def add_direktmandat(self):
        self.direktmandate += 1
        self.partei.direktmandate += 1

    def update_stimmen(self, erststimmen, zweitstimmen):
        self.erststimmen = erststimmen
        self.zweitstimmen = zweitstimmen
        self.partei.update_zweitstimmen(zweitstimmen)
    
    def __str__(self):
        return "{" + self.name + "}"
    def __repr__(self):
        return self.__str__()
