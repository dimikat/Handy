import React from "react";

export interface Tab {
  id: string;
  label: string;
  icon?: React.ReactNode;
  badge?: string | number;
}

interface TabNavigationProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  className?: string;
}

export const TabNavigation: React.FC<TabNavigationProps> = ({
  tabs,
  activeTab,
  onTabChange,
  className = "",
}) => {
  return (
    <div className={`border-b border-mid-gray/20 ${className}`}>
      <nav className="flex space-x-0">
        {tabs.map((tab) => {
          const isActive = tab.id === activeTab;
          return (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className={`
                relative px-6 py-3 text-sm font-medium transition-all duration-200 ease-in-out
                border-b-2 border-transparent hover:text-logo-primary
                ${
                  isActive
                    ? "text-logo-primary border-logo-primary bg-logo-primary/5"
                    : "text-mid-gray hover:border-logo-primary/50"
                }
              `}
            >
              <div className="flex items-center gap-2">
                {tab.icon && (
                  <span className="w-4 h-4 flex items-center justify-center">
                    {tab.icon}
                  </span>
                )}
                <span>{tab.label}</span>
                {tab.badge && (
                  <span className="ml-1 px-2 py-0.5 text-xs bg-logo-primary/20 text-logo-primary rounded-full">
                    {tab.badge}
                  </span>
                )}
              </div>
            </button>
          );
        })}
      </nav>
    </div>
  );
};