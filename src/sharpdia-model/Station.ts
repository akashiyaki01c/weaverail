import {
  OuDia_Eki,
  OuDia_Ekijikokukeisiki,
  OuDia_Ekikibo,
  OuDia_EkiTrack2,
} from '../oudia-parser/oudia-model/Eki';

/** 駅を表す */
export class Station {
  id: string;
  constructor(
    public name: string,
    public displayType: OuDia_Ekijikokukeisiki,
    public kibo: OuDia_Ekikibo,
    public tracks: Track[],
  ) {
    this.id = crypto.randomUUID();
  }

  static default() {
    return new Station('', 'Hatsu', 'Ippan', []);
  }
  static fromOuDia(value: OuDia_Eki) {
    const object = new Station(
      value.ekimei,
      value.ekijikokukeisiki,
      value.ekikibo,
      value.track.value.map((v) => Track.fromOuDia(v)),
    );
    return object;
  }
}

/** 番線を表す */
export class Track {
  id: string;
  constructor(public name: string) {
    this.id = crypto.randomUUID();
  }

  static fromOuDia(value: OuDia_EkiTrack2) {
    const object = new Track(value.trackName);
    return object;
  }
}
