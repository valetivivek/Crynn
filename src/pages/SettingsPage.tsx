import React from "react";
import { Settings, X } from "lucide-react";
import ThemeSettings from "../components/Settings/ThemeSettings";
import KeybindingSettings from "../components/Settings/KeybindingSettings";
import PrivacySettings from "../components/Settings/PrivacySettings";

interface SettingsPageProps {
  onClose: () => void;
}

const SettingsPage: React.FC<SettingsPageProps> = ({ onClose }) => {
  return (
    <div className="fixed inset-0 bg-bg z-50 flex flex-col">
      <div className="h-12 border-b border-border flex items-center justify-between px-4">
        <div className="flex items-center gap-2">
          <Settings className="w-5 h-5" />
          <h1 className="text-lg font-medium">Settings</h1>
        </div>
        <button
          onClick={onClose}
          className="p-1.5 hover:bg-muted/10 rounded"
        >
          <X className="w-5 h-5" />
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-2xl mx-auto space-y-8">
          <ThemeSettings />
          <KeybindingSettings />
          <PrivacySettings />
        </div>
      </div>
    </div>
  );
};

export default SettingsPage;

