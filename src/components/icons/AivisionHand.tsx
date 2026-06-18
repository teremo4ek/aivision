const AivisionHand = ({
  width,
  height,
}: {
  width?: number | string;
  height?: number | string;
}) => (
  <svg
    width={width || 126}
    height={height || 135}
    viewBox="0 0 126 135"
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    stroke="currentColor"
  >
    <path
      d="M15 67.5
          C30 48 48 40 63 40
          C78 40 96 48 111 67.5
          C96 87 78 95 63 95
          C48 95 30 87 15 67.5 Z"
      strokeWidth="9"
      strokeLinecap="round"
      strokeLinejoin="round"
    />

    {/* кольцо */}
    <circle
      cx="63"
      cy="67.5"
      r="14"
      strokeWidth="8"
    />

    <circle
      cx="63"
      cy="67.5"
      r="6"
      fill="currentColor"
      stroke="none"
    />

    <path d="M47 51H56 M47 51V60" strokeWidth="7" strokeLinecap="round"/>
    <path d="M79 51H70 M79 51V60" strokeWidth="7" strokeLinecap="round"/>
    <path d="M47 84H56 M47 84V75" strokeWidth="7" strokeLinecap="round"/>
    <path d="M79 84H70 M79 84V75" strokeWidth="7" strokeLinecap="round"/>
  </svg>
);

export default AivisionHand;
