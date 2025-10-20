import { DiagramViewerOption } from './DiagramViewer';

export function XAxisSvg({ option }: { option: DiagramViewerOption }) {
  const viewBoxWidth = option.xScale * 60 * 60 * 24;

  return (
    <svg
      className="x-axis-svg overflow-scroll sticky top-[0] block"
      height={50}
      viewBox={`0 0 ${viewBoxWidth} ${50}`}
      width={viewBoxWidth}
    >
      {Array.from({ length: 24 * 60 + 1 })
        .map((_, index) => index)
        .filter((v) => v % 60 === 0)
        .map((v) => {
          let time = v + 4 * 60;
          if (time < 0) {
            time += 24 * 60;
          }
          return (
            <text
              data-key={`xaxis-${time}-1`}
              dominantBaseline="middle"
              fill="#000"
              fontSize={32}
              id={`xaxis-${time}-1`}
              key={`xaxis-${time}-1`}
              textAnchor="middle"
              x={(option.xOffset + v * 60) * option.xScale}
              y={25}
            >
              {(time / 60) % 24}
            </text>
          );
        })}
    </svg>
  );
}
