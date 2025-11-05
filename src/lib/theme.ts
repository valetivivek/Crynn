export const applyTheme = (theme: "light" | "dark" | "auto" | "highContrast") => {
  const root = document.documentElement;
  
  if (theme === "auto") {
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.setAttribute("data-theme", prefersDark ? "dark" : "light");
    
    // Listen for changes
    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", (e) => {
      root.setAttribute("data-theme", e.matches ? "dark" : "light");
    });
  } else {
    root.setAttribute("data-theme", theme);
  }
};

