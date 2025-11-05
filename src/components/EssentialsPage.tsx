import React from "react";
import { useTabsStore } from "../stores/tabsStore";

interface EssentialsPageProps {
  onNavigate: (url: string) => void;
}

const EssentialsPage: React.FC<EssentialsPageProps> = ({ onNavigate }) => {
  const quickLinks = [
    { name: "Google", url: "https://www.google.com", icon: "🔍", color: "bg-blue-500/10 hover:bg-blue-500/20" },
    { name: "YouTube", url: "https://www.youtube.com", icon: "📺", color: "bg-red-500/10 hover:bg-red-500/20" },
    { name: "GitHub", url: "https://github.com", icon: "💻", color: "bg-gray-500/10 hover:bg-gray-500/20" },
    { name: "Twitter", url: "https://twitter.com", icon: "🐦", color: "bg-sky-500/10 hover:bg-sky-500/20" },
    { name: "Reddit", url: "https://www.reddit.com", icon: "🤖", color: "bg-orange-500/10 hover:bg-orange-500/20" },
    { name: "Wikipedia", url: "https://www.wikipedia.org", icon: "📚", color: "bg-gray-500/10 hover:bg-gray-500/20" },
  ];

  const handleClick = (url: string) => {
    onNavigate(url);
  };

  return (
    <div className="w-full h-full flex items-center justify-center bg-bg p-8">
      <div className="max-w-4xl w-full">
        <h1 className="text-3xl font-light mb-2 text-fg">Welcome to Crynn</h1>
        <p className="text-muted mb-8">Quick access to your essentials</p>
        
        <div className="grid grid-cols-3 gap-4 mb-8">
          {quickLinks.map((link) => (
            <button
              key={link.url}
              onClick={() => handleClick(link.url)}
              className={`${link.color} p-6 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95 text-left group`}
            >
              <div className="text-3xl mb-2">{link.icon}</div>
              <div className="text-sm font-medium text-fg">{link.name}</div>
            </button>
          ))}
        </div>

        <div className="text-center text-sm text-muted">
          <p>Press Ctrl/Cmd+T to open a new tab</p>
          <p className="mt-2">Or type in the address bar to search or navigate</p>
        </div>
      </div>
    </div>
  );
};

export default EssentialsPage;

