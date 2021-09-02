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

        self.mindestsitzzahl = None
        self.sitze = None
        self.mindestsitzanspruch = None
        self.ueberhang = None
        self.percentageZweit = None

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
    def direktmandate(self):
        return self._direktmandate
    @direktmandate.setter
    def direktmandate(self, direktmandate):
        if isinstance(direktmandate, int):
            self._direktmandate = direktmandate
        else:
            raise AttributeError("direktmandate has to be an int, but was", type(direktmandate))
    @property
    def ueberHuerde(self):
        return self._ueberHuerde
    @ueberHuerde.setter
    def ueberHuerde(self, ueberHuerde):
        if isinstance(ueberHuerde, bool):
            self._ueberHuerde = ueberHuerde
        else:
            raise AttributeError("ueberHuerde has to be a bool, but was", type(ueberHuerde))
    @property
    def partei_in_land(self):
        return self._partei_in_land
    @partei_in_land.setter
    def partei_in_land(self, partei_in_land):
        if isinstance(partei_in_land, dict):
            self._partei_in_land = partei_in_land
        else:
            raise AttributeError("partei_in_land has to be a dict, but was", type(partei_in_land))
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
    def mindestsitzzahl(self):
        return self._mindestsitzzahl
    @mindestsitzzahl.setter
    def mindestsitzzahl(self, mindestsitzzahl):
        if isinstance(mindestsitzzahl, int) or mindestsitzzahl is None:
            self._mindestsitzzahl = mindestsitzzahl
        else:
            raise AttributeError("mindestsitzzahl has to be an int, but was", type(mindestsitzzahl))
    @property
    def sitze(self):
        return self._sitze
    @sitze.setter
    def sitze(self, sitze):
        if isinstance(sitze, int) or sitze is None:
            self._sitze = sitze
        else:
            raise AttributeError("sitze has to be an int, but was", type(sitze))
    @property
    def mindestsitzanspruch(self):
        return self._mindestsitzanspruch
    @mindestsitzanspruch.setter
    def mindestsitzanspruch(self, mindestsitzanspruch):
        if isinstance(mindestsitzanspruch, int) or mindestsitzanspruch is None:
            self._mindestsitzanspruch = mindestsitzanspruch
        else:
            raise AttributeError("mindestsitzanspruch has to be an int, but was", type(mindestsitzanspruch))
    @property
    def ueberhang(self):
        return self._ueberhang
    @ueberhang.setter
    def ueberhang(self, ueberhang):
        if isinstance(ueberhang, int) or ueberhang is None:
            self._ueberhang = ueberhang
        else:
            raise AttributeError("ueberhang has to be an int, but was", type(ueberhang))
    @property
    def percentageZweit(self):
        return self._percentageZweit
    @percentageZweit.setter
    def percentageZweit(self, percentageZweit):
        if isinstance(percentageZweit, float) or percentageZweit is None:
            self._percentageZweit = percentageZweit
        else:
            raise AttributeError("percentageZweit has to be a float, but was", type(percentageZweit))

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
