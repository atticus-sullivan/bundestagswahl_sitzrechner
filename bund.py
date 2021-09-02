from decimal import Decimal

class Bund:
    def __init__(self):
        self.parteien = {}
        self.laender = {}

    def __str__(self):
        return "{[Bund] parteien:" + str(self.parteien) + ", laender:" + str(self.laender) + "}"
    def __repr__(self):
        return str(self)

    @property
    def parteien(self):
        return self._parteien
    @parteien.setter
    def parteien(self, parteien):
        if isinstance(parteien, dict):
            self._parteien = parteien
        else:
            raise AttributeError("parteien has to be a dict, but was", type(parteien))

    @property
    def laender(self):
        return self._laender
    @laender.setter
    def laender(self, laender):
        if isinstance(laender, dict):
            self._laender = laender
        else:
            raise AttributeError("laender has to be a dict, but was", type(laender))

    def add_partei(self, partei):
        self.parteien.update({partei.name: partei})

    def add_land(self, land):
        self.laender.update({land.name: land})

    def calc_percentageHuerde(self):
        total = 0
        for p in self.parteien.values():
            total += p.zweitstimmen

        for p in self.parteien.values():
            p.percentageZweit = Decimal(p.zweitstimmen) / total
            if p.percentageZweit > 0.05:
                p.ueberHuerde = True

    def huerde_remove(self):
        for l in self.laender.values():
            for n,p in list(l.parteien.items()):
                if not p.partei.ueberHuerde and p.partei.direktmandate < 3:
                    del l.parteien[n]
        for n,p in list(self.parteien.items()):
            if not p.ueberHuerde and p.direktmandate < 3:
                del self.parteien[n]
