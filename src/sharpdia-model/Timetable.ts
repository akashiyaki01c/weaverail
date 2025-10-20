import { Train } from './Train';

export class Passing {
  id: string;

  constructor(
    public stationId: string,
    public stoppingTrainId: string,
    public passingTrainId: string,
  ) {
    this.id = crypto.randomUUID();
  }
}

export class Timetable {
  id: string;

  constructor(
    public name: string,
    public trains: Train[],
    public passings: Passing[],
  ) {
    this.id = crypto.randomUUID();
  }

  static default() {
    return new Timetable('', [], []);
  }
}
