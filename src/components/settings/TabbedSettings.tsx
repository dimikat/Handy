import React, { useState } from "react";
import { TabNavigation, Tab } from "../ui/TabNavigation";
import { ShortcutProfiles } from "./ShortcutProfiles";
import { MicrophoneSelector } from "./MicrophoneSelector";
import { AlwaysOnMicrophone } from "./AlwaysOnMicrophone";
import { PushToTalk } from "./PushToTalk";
import { AudioFeedback } from "./AudioFeedback";
import { OutputDeviceSelector } from "./OutputDeviceSelector";
import { TranslateToEnglish } from "./TranslateToEnglish";
import { SettingsGroup } from "../ui/SettingsGroup";

// Tab icons (using simple SVG icons)
const ShortcutIcon = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z" />
  </svg>
);

const AudioIcon = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
  </svg>
);

const ModelsIcon = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
  </svg>
);

const AdvancedIcon = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4" />
  </svg>
);

export const TabbedSettings: React.FC = () => {
  const [activeTab, setActiveTab] = useState("shortcuts");

  const tabs: Tab[] = [
    {
      id: "shortcuts",
      label: "Shortcuts",
      icon: <ShortcutIcon />,
    },
    {
      id: "audio",
      label: "Audio",
      icon: <AudioIcon />,
    },
    {
      id: "models",
      label: "Models",
      icon: <ModelsIcon />,
    },
    {
      id: "advanced",
      label: "Advanced",
      icon: <AdvancedIcon />,
    },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case "shortcuts":
        return (
          <div className="space-y-6">
            <SettingsGroup>
              <ShortcutProfiles />
            </SettingsGroup>
          </div>
        );
      
      case "audio":
        return (
          <div className="space-y-6">
            <SettingsGroup title="Input">
              <MicrophoneSelector descriptionMode="tooltip" grouped={true} />
              <AlwaysOnMicrophone descriptionMode="tooltip" grouped={true} />
              <PushToTalk descriptionMode="tooltip" grouped={true} />
            </SettingsGroup>
            
            <SettingsGroup title="Output">
              <OutputDeviceSelector descriptionMode="tooltip" grouped={true} />
              <AudioFeedback descriptionMode="tooltip" grouped={true} />
            </SettingsGroup>
          </div>
        );
      
      case "models":
        return (
          <div className="space-y-6">
            <SettingsGroup>
              <div className="p-4 text-center text-mid-gray">
                <p>Model management will be implemented here</p>
                <p className="text-sm mt-2">This will include model selection, downloading, and switching</p>
              </div>
            </SettingsGroup>
          </div>
        );
      
      case "advanced":
        return (
          <div className="space-y-6">
            <SettingsGroup title="Language & Translation">
              <TranslateToEnglish descriptionMode="tooltip" grouped={true} />
            </SettingsGroup>
            
            <SettingsGroup title="Other Settings">
              <div className="p-4 text-center text-mid-gray">
                <p>Additional advanced settings will be added here</p>
              </div>
            </SettingsGroup>
          </div>
        );
      
      default:
        return null;
    }
  };

  return (
    <div className="max-w-4xl w-full mx-auto">
      <TabNavigation
        tabs={tabs}
        activeTab={activeTab}
        onTabChange={setActiveTab}
        className="mb-6"
      />
      
      <div className="min-h-[400px]">
        {renderTabContent()}
      </div>
    </div>
  );
};