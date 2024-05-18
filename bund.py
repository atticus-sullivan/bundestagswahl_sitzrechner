# Copyright (c) 2024, Lukas Heindl
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its
#    contributors may be used to endorse or promote products derived from
#    this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
