/*
 * Doman Scale game.
 * Copyright (C) 2023 Ghostkeeper
 * This application is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This application is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for details.
 * You should have received a copy of the GNU Affero General Public License along with this application. If not, see <https://gnu.org/licenses/>.
 */

/// The musical instruments that are available.
///
/// This simply lists all of the MIDI instruments.
pub enum Instrument {
	//Piano.
	GrandPiano = 0,
	BrightPiano = 1,
	ElectricGrandPiano = 2,
	HonkyTonkPiano = 3,
	ElectricPiano1 = 4,
	ElectricPiano2 = 5,
	Harpsichord = 6,
	Clavinet = 7,

	//Chromatic percussion.
	Celesta = 8,
	Glockenspiel = 9,
	MusicBox = 10,
	Vibraphone = 11,
	Marimba = 12,
	Xylophone = 13,
	TubularBells = 14,
	Dulcimer = 15,

	//Organ.
	DrawbarOrgan = 16,
	PercussiveOrgan = 17,
	RockOrgan = 18,
	ChurchOrgan = 19,
	ReedOrgan = 20,
	Accordion = 21,
	Harmonica = 22,
	TangoAccordion = 23,

	//Guitar.
	NylonGuitar = 24,
	SteelGuitar = 25,
	JazzGuitar = 26,
	CleanGuitar = 27,
	MutedGuitar = 28,
	OverdrivenGuitar = 29,
	DistortionGuitar = 30,
	GuitarHarmonics = 31,

	//Bass.
	AcousticBass = 32,
	FingerBass = 33,
	PickBass = 34,
	FretlessBass = 35,
	SlapBass1 = 36,
	SlapBass2 = 37,
	SynthBass1 = 38,
	SynthBass2 = 39,

	//Strings.
	Violin = 40,
	Viola = 41,
	Cello = 42,
	Contrabass = 43,
	Tremolo = 44,
	Pizzicato = 45,
	Harp = 46,
	Timpani = 47,
	StringEnsemble1 = 48,
	StringEnsemble2 = 49,
	SynthStrings1 = 50,
	SynthStrings2 = 51,
	ChoirAahs = 52,
	VoiceOohs = 53,
	SynthVoice = 54,
	OrchestraHit = 55,

	//Brass.
	Trumpet = 56,
	Trombone = 57,
	Tuba = 58,
	MutedTrumpet = 59,
	FrenchHorn = 60,
	BrassSection = 61,
	SynthBrass1 = 62,
	SynthBrass2 = 63,

	//Reed.
	SopranoSax = 64,
	AltoSax = 65,
	TenorSax = 66,
	BaritoneSax = 67,
	Oboe = 68,
	EnglishHorn = 69,
	Bassoon = 70,
	Clarinet = 71,

	//Pipe.
	Piccolo = 72,
	Flute = 73,
	Recorder = 74,
	PanFlute = 75,
	Bottle = 76,
	Shakuhachi = 77,
	Whistle = 78,
	Ocarina = 79,

	//Synth lead.
	Square = 80,
	Sawtooth = 81,
	Calliope = 82,
	Chiff = 83,
	Charang = 84,
	SynthLeadVoice = 85,
	Fifths = 86,
	BassLead = 87,

	//Synth pad.
	NewAge = 88,
	WarmPad = 89,
	Polysynth = 90,
	ChoirPad = 91,
	Bowed = 92,
	Metallic = 93,
	Halo = 94,
	Sweep = 95,

	//Synth effects.
	Rain = 96,
	Soundtrack = 97,
	Crystal = 98,
	Atmosphere = 99,
	Brightness = 100,
	Goblins = 101,
	Echoes = 102,
	SciFi = 103,

	//Ethnic.
	Sitar = 104,
	Banjo = 105,
	Shamisen = 106,
	Koto = 107,
	Kalimba = 108,
	BagPipe = 109,
	Fiddle = 110,
	Shanai = 111,

	//Percussive.
	TinkleBell = 112,
	Agogo = 113,
	SteelDrums = 114,
	Woodblock = 115,
	TaikoDrum = 116,
	MelodicTom = 117,
	SynthDrum = 118,

	//Sound effects.
	ReverseCymbal = 119,
	GuitarFretNoise = 120,
	BreathNoise = 121,
	Seashore = 122,
	BirdTweet = 123,
	TelephoneRing = 124,
	Helicopter = 125,
	Applause = 126,
	Gunshot = 127
}