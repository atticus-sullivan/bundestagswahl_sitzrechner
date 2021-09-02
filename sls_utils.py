"""
One implementation of how seats can be distributed
"""
class Sainte_Lague_Schepers_utils:
    def __init__(self, zweitwahl, ges_sitze):
        self.zweitwahl = zweitwahl
        self.ges_sitze = ges_sitze
        self.verteilung = {}

    def __str__(self):
        return "{[sls_utils] zweitwahl:" + str(self.zweitwahl) + ", ges_sitze:" + str(self.ges_sitze) + ", verteilung:" + str(self.verteilung) + "}"
    def __repr__(self):
        return str(self)

    @property
    def zweitwahl(self):
        return self._zweitwahl
    @zweitwahl.setter
    def zweitwahl(self, zweitwahl):
        if isinstance(zweitwahl, dict):
            self._zweitwahl = zweitwahl
        else:
            raise AttributeError("zweitwahl has to be a dict, but was", type(zweitwahl))
    @property
    def ges_sitze(self):
        return self._ges_sitze
    @ges_sitze.setter
    def ges_sitze(self, ges_sitze):
        if isinstance(ges_sitze, int):
            self._ges_sitze = ges_sitze
        else:
            raise AttributeError("ges_sitze has to be an int, but was", type(ges_sitze))
    @property
    def verteilung(self):
        return self._verteilung
    @verteilung.setter
    def verteilung(self, verteilung):
        if isinstance(verteilung, dict):
            self._verteilung = verteilung
        else:
            raise AttributeError("verteilung has to be a dict, but was", type(verteilung))

    """
    calculate distribution
    """
    def apply(self):
        zutDivisor = sum([x for _,x in self.zweitwahl.items()]) / self.ges_sitze
        while (s := sum([x for _,x in self.verteilung.items()])) != self.ges_sitze: #TODO passen die steps so, oder kann man die irgendwie schlau berechnen (oder immer +-10% oder so) check ob osziliert -> steps kleiner machen (dann aber bei 100 anfangen)
            if s < self.ges_sitze:
                zutDivisor -= 1
            elif s > self.ges_sitze:
                zutDivisor += 1
            self.updateVert(zutDivisor)
        # print("Divisor:", zutDivisor, self.verteilung)
        return self.verteilung

    def updateVert(self, zutDivisor):
        for p, stimmen in self.zweitwahl.items():
            sitze = round(stimmen / zutDivisor) #TODO .5 -> würfeln nicht beachtet
            self.verteilung.update({p: sitze})
