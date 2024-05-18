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

    @classmethod
    def clear(cls):
        cls.parteien = {}

    def __str__(self):
        return "[Partei] name:" + str(self.name) + ", direktmandate:" + str(self.direktmandate) + ", ueberHuerde: " + str(self.ueberHuerde) + ", partei_in_land:" + str(self.partei_in_land) + ", zweitstimmen:" + str(self.zweitstimmen) + ", mindestsitzzahl:" + str(self.mindestsitzzahl) + ", sitze:" + str(self.sitze) + ", mindestsitzanspruch:" + str(self.mindestsitzanspruch) + ", ueberhang:" + str(self.ueberhang) + ", perctnageZweit:" + str(self.percentageZweit) + "}"
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
        if isinstance(percentageZweit, Decimal) or percentageZweit is None:
            self._percentageZweit = percentageZweit
        else:
            raise AttributeError("percentageZweit has to be a Decimal, but was", type(percentageZweit))

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

    def add(self, partei_in_land):
        self.partei_in_land.update({partei_in_land.land.name: partei_in_land})
