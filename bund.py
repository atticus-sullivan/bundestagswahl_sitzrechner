class Bund:
    def __init__(self):
        self.parteien = {}
        self.laender = {}

    def add_partei(self, partei):
        self.parteien.update({partei.name: partei})

    def add_land(self, land):
        self.laender.update({land.name: land})

    def calc_percentageHuerde(self):
        total = 0
        for p in self.parteien.values():
            total += p.zweitstimmen

        for p in self.parteien.values():
            if p.zweitstimmen / total > 0.05:
                p.ueberHuerde = True

    def huerde_remove(self):
        for l in self.laender.values():
            for n,p in list(l.parteien.items()):
                if not p.partei.ueberHuerde and p.partei.direktmandate < 3:
                    del l.parteien[n]
        for n,p in list(self.parteien.items()):
            if not p.ueberHuerde and p.direktmandate < 3:
                del self.parteien[n]
