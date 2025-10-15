import { DiagramLine } from './DiagramLine';
import { Line } from './Line';
import { Station } from './Station';
import { TemplateTrain } from './TemplateTrain';
import { Timetable } from './Timetable';
import { TrainType } from './TrainType';

export class Root {
  constructor(
    public stations: Station[] = [],
    public lines: Line[] = [],
    public trainTypes: TrainType[] = [],
    public timetables: Timetable[] = [],
    public diagramLines: DiagramLine[] = [],
    public templateTrains: TemplateTrain[] = [],
  ) {}
}
