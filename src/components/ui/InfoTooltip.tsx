import React, { useState, useEffect, useRef, ReactNode } from "react";

interface InfoTooltipProps {
  content: ReactNode;
  children?: ReactNode;
  placement?: "top" | "bottom" | "left" | "right";
  maxWidth?: string;
  showOnHover?: boolean;
  showOnClick?: boolean;
  className?: string;
}

export const InfoTooltip: React.FC<InfoTooltipProps> = ({
  content,
  children,
  placement = "top",
  maxWidth = "320px",
  showOnHover = true,
  showOnClick = true,
  className = "",
}) => {
  const [isVisible, setIsVisible] = useState(false);
  const tooltipRef = useRef<HTMLDivElement>(null);

  // Handle click outside to close tooltip
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        tooltipRef.current &&
        !tooltipRef.current.contains(event.target as Node)
      ) {
        setIsVisible(false);
      }
    };

    if (isVisible && showOnClick) {
      document.addEventListener("mousedown", handleClickOutside);
      return () =>
        document.removeEventListener("mousedown", handleClickOutside);
    }
  }, [isVisible, showOnClick]);

  const toggleTooltip = () => {
    if (showOnClick) {
      setIsVisible(!isVisible);
    }
  };

  const handleMouseEnter = () => {
    if (showOnHover) {
      setIsVisible(true);
    }
  };

  const handleMouseLeave = () => {
    if (showOnHover) {
      setIsVisible(false);
    }
  };

  const getPlacementClasses = () => {
    switch (placement) {
      case "top":
        return "bottom-full left-1/2 transform -translate-x-1/2 mb-2";
      case "bottom":
        return "top-full left-1/2 transform -translate-x-1/2 mt-2";
      case "left":
        return "right-full top-1/2 transform -translate-y-1/2 mr-2";
      case "right":
        return "left-full top-1/2 transform -translate-y-1/2 ml-2";
      default:
        return "bottom-full left-1/2 transform -translate-x-1/2 mb-2";
    }
  };

  const getArrowClasses = () => {
    switch (placement) {
      case "top":
        return "absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-[6px] border-r-[6px] border-t-[6px] border-l-transparent border-r-transparent border-t-background";
      case "bottom":
        return "absolute bottom-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-[6px] border-r-[6px] border-b-[6px] border-l-transparent border-r-transparent border-b-background";
      case "left":
        return "absolute left-full top-1/2 transform -translate-y-1/2 w-0 h-0 border-t-[6px] border-b-[6px] border-l-[6px] border-t-transparent border-b-transparent border-l-background";
      case "right":
        return "absolute right-full top-1/2 transform -translate-y-1/2 w-0 h-0 border-t-[6px] border-b-[6px] border-r-[6px] border-t-transparent border-b-transparent border-r-background";
      default:
        return "absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-[6px] border-r-[6px] border-t-[6px] border-l-transparent border-r-transparent border-t-background";
    }
  };

  const defaultTrigger = (
    <svg
      className="w-4 h-4 text-mid-gray cursor-help hover:text-logo-primary transition-colors duration-200 select-none"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      aria-label="More information"
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          toggleTooltip();
        }
      }}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  );

  return (
    <div ref={tooltipRef} className={`relative inline-block ${className}`}>
      <div
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        onClick={toggleTooltip}
      >
        {children || defaultTrigger}
      </div>
      
      {isVisible && (
        <div
          className={`
            absolute z-50 px-4 py-3 bg-background border border-mid-gray/80 
            rounded-lg shadow-lg whitespace-normal animate-in fade-in-0 zoom-in-95 duration-200
            ${getPlacementClasses()}
          `}
          style={{ maxWidth }}
        >
          <div className="text-sm leading-relaxed">
            {content}
          </div>
          <div className={getArrowClasses()}></div>
        </div>
      )}
    </div>
  );
};