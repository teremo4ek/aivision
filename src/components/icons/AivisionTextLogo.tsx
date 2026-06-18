import React from "react";

const AivisionTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
<svg
  width={width}
  height={height}
  className={className}
  viewBox="0 0 930 328"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
  stroke="currentColor"
>
  {/* A */}
  <path
    d="M44 255 L88 73 L133 255 M62 189 H115"
    strokeWidth="18"
    strokeLinecap="round"
    strokeLinejoin="round"
  />

  {/* i */}
  <circle cx="182" cy="88" r="10" fill="currentColor" stroke="none" />
  <path
    d="M182 138 V255"
    strokeWidth="18"
    strokeLinecap="round"
  />

  {/* V */}
  <path
    d="M244 73 L288 255 L333 73"
    strokeWidth="18"
    strokeLinecap="round"
    strokeLinejoin="round"
  />

  {/* i */}
  <circle cx="382" cy="88" r="10" fill="currentColor" stroke="none" />
  <path
    d="M382 138 V255"
    strokeWidth="18"
    strokeLinecap="round"
  />

  {/* s */}
  <path
    d="M458 153
       C436 124 400 138 400 175
       C400 211 458 197 458 240
       C458 269 422 277 400 248"
    strokeWidth="18"
    strokeLinecap="round"
    strokeLinejoin="round"
  />

  {/* i */}
  <circle cx="511" cy="88" r="10" fill="currentColor" stroke="none" />
  <path
    d="M511 138 V255"
    strokeWidth="18"
    strokeLinecap="round"
  />

  {/* o */}
  <circle
    cx="600"
    cy="197"
    r="53"
    strokeWidth="18"
  />

  {/* n */}
  <path
    d="M703 255 V138
       M703 182
       C732 124 806 124 806 197
       V255"
    strokeWidth="18"
    strokeLinecap="round"
    strokeLinejoin="round"
  />
</svg>
  );
};

export default AivisionTextLogo;
