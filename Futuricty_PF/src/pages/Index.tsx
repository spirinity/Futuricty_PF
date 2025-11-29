import React, { useState, useCallback, useEffect } from "react";
import { toast } from "sonner";
import Map from "@/components/Map";
import LocationSearch from "@/components/LocationSearch";
import LiveabilityScore from "@/components/LiveabilityScore";
import ControlPanel from "@/components/ControlPanel";
import { ThemeToggle } from "@/components/ThemeToggle";
import LanguageToggle from "@/components/LanguageToggle";
import {
  calculateLivabilityScore,
  getEmptyLivabilityData,
} from "@/services/livabilityService";
import { searchHistoryService } from "@/services/searchHistoryService";
import { Menu, BarChart3, X, MapPin, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useLanguage } from "@/components/LanguageProvider";

const Index = () => {
  const { t } = useLanguage();
  const [selectedLocations, setSelectedLocations] = useState<
    Array<{ lng: number; lat: number; address?: string }>
  >([]);
  const [activeLocationIndex, setActiveLocationIndex] = useState(0);
  const [analysisResults, setAnalysisResults] = useState<any[]>([]);

  // Keep these for backward compatibility with child components if needed,
  // but we will derive them from analysisResults[activeLocationIndex]
  const [livabilityData, setLivabilityData] = useState(
    getEmptyLivabilityData()
  );
  const [facilities, setFacilities] = useState<any[]>([]);

  const [isCalculating, setIsCalculating] = useState(false);
  const [showRadius, setShowRadius] = useState(true);
  const [hasCalculated, setHasCalculated] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [bottomSheetHeight, setBottomSheetHeight] = useState(120); // Default collapsed height
  const [isDragging, setIsDragging] = useState(false);
  const [dragStartY, setDragStartY] = useState<number | null>(null);
  const [dragStartHeight, setDragStartHeight] = useState<number>(0);
  const [isBottomSheetVisible, setIsBottomSheetVisible] = useState(true);
  const radiusOptions = [250, 500];

  // Facility category visibility state
  const [visibleCategories, setVisibleCategories] = useState<
    Record<string, boolean>
  >({
    health: true,
    education: true,
    market: true,
    transport: true,
    walkability: true,
    safety: true,
    accessibility: true,
    police: true,
    religious: true,
    recreation: true,
  });

  // Control panel visibility state
  const [isControlPanelVisible, setIsControlPanelVisible] = useState(true);

  // Update current view data when active index or results change
  useEffect(() => {
    if (hasCalculated && analysisResults.length > activeLocationIndex) {
      setLivabilityData(analysisResults[activeLocationIndex].data);
      setFacilities(analysisResults[activeLocationIndex].facilities);
    } else {
      setLivabilityData(getEmptyLivabilityData());
      setFacilities([]);
    }
  }, [activeLocationIndex, analysisResults, hasCalculated]);

  const handleToggleCategory = useCallback((category: string) => {
    setVisibleCategories((prev) => ({
      ...prev,
      [category]: !prev[category],
    }));
  }, []);

  const handleToggleAllCategories = useCallback(() => {
    const allHidden = Object.values(visibleCategories).every((v) => !v);

    setVisibleCategories((prev) => {
      const updated = { ...prev };
      Object.keys(updated).forEach((category) => {
        updated[category] = allHidden; // If all hidden, show all. Otherwise, hide all
      });
      return updated;
    });
  }, [visibleCategories]);

  const handleLocationSelect = useCallback(
    (lng: number, lat: number, address?: string) => {
      // If we already have results, clear them and start over
      if (hasCalculated) {
        setHasCalculated(false);
        setAnalysisResults([]);
        setSelectedLocations([{ lng, lat, address }]);
        setActiveLocationIndex(0);
      } else {
        // Add new location if less than 3
        setSelectedLocations((prev) => {
          if (prev.length >= 3) {
            toast.error("Maximum 3 locations allowed");
            return prev;
          }
          return [...prev, { lng, lat, address }];
        });
      }

      if (address) {
        searchHistoryService.addToHistory(address, address, { lat, lng });
      }
    },
    [hasCalculated, selectedLocations.length]
  );

  const removeLocation = useCallback((index: number) => {
    setSelectedLocations((prev) => prev.filter((_, i) => i !== index));
    setHasCalculated(false);
    setAnalysisResults([]);
  }, []);

  const handleRecalculate = useCallback(async () => {
    if (selectedLocations.length !== 3) {
      toast.error(t("pleaseSelect3Locations"), {
        description: t("mustSelect3LocationsDescription"),
      });
      return;
    }

    setIsCalculating(true);

    try {
      setAnalysisResults([]);

      // Call API with all locations
      const locationsToAnalyze = selectedLocations.map((loc) => ({
        ...loc,
        address: loc.address || `${loc.lat.toFixed(4)}, ${loc.lng.toFixed(4)}`,
      }));

      const results = await calculateLivabilityScore(locationsToAnalyze);

      setAnalysisResults(results);
      setHasCalculated(true);
      setActiveLocationIndex(0);

      toast.success(t("livability.score.calculated", { score: "" }), {
        description: `Analyzed ${results.length} locations successfully.`,
      });
    } catch (error) {
      toast.error(t("failed.calculate.score"), {
        description: t("try.again.select.different"),
      });
    } finally {
      setIsCalculating(false);
    }
  }, [selectedLocations, t]);

  const handleAnalyzeLocation = useCallback(async () => {
    handleRecalculate();
  }, [handleRecalculate]);

  const handleToggleRadius = useCallback(() => {
    setShowRadius((prev) => !prev);
  }, []);

  // Control panel toggle handler
  const toggleControlPanel = useCallback(() => {
    setIsControlPanelVisible((prev) => !prev);
  }, []);

  const toggleMobileMenu = () => {
    setIsMobileMenuOpen(!isMobileMenuOpen);
  };

  // Bottom sheet gesture handlers for mobile - Optimized for performance
  const handleBottomSheetTouchStart = useCallback(
    (e: React.TouchEvent) => {
      // Only start dragging if touching the handle area
      const target = e.target as HTMLElement;
      if (!target.closest(".mobile-bottom-sheet-handle")) {
        return;
      }

      e.preventDefault();
      e.stopPropagation();
      setDragStartY(e.touches[0].clientY);
      setDragStartHeight(bottomSheetHeight);
      setIsDragging(true);
    },
    [bottomSheetHeight]
  );

  const handleBottomSheetTouchMove = useCallback(
    (e: React.TouchEvent) => {
      if (!isDragging || dragStartY === null) return;

      e.preventDefault();
      e.stopPropagation();

      // Use requestAnimationFrame for smooth updates
      requestAnimationFrame(() => {
        const currentY = e.touches[0].clientY;
        const deltaY = dragStartY - currentY;
        const newHeight = Math.max(
          0,
          Math.min(window.innerHeight - 100, dragStartHeight + deltaY)
        );

        setBottomSheetHeight(newHeight);
      });
    },
    [isDragging, dragStartY, dragStartHeight]
  );

  const handleBottomSheetTouchEnd = useCallback(
    (e: React.TouchEvent) => {
      if (!isDragging) return;

      e.preventDefault();
      e.stopPropagation();
      setIsDragging(false);
      setDragStartY(null);

      // Snap to nearest position or close completely
      const currentHeight = bottomSheetHeight;
      let targetHeight: number;

      if (currentHeight < 60) {
        // Close the bottom sheet completely
        targetHeight = 0;
        setIsBottomSheetVisible(false);
      } else if (currentHeight > window.innerHeight * 0.7) {
        // Snap to almost full screen
        targetHeight = window.innerHeight - 100;
      } else if (currentHeight > window.innerHeight * 0.4) {
        // Snap to expanded (about 60% of screen)
        targetHeight = window.innerHeight * 0.6;
      } else if (currentHeight > 150) {
        // Snap to medium height
        targetHeight = 200;
      } else {
        // Snap to collapsed
        targetHeight = 120;
      }

      setBottomSheetHeight(targetHeight);
    },
    [isDragging, bottomSheetHeight]
  );

  // Swipe up to expand bottom sheet - Optimized
  const handleSwipeUp = useCallback(() => {
    if (bottomSheetHeight < 200) {
      setBottomSheetHeight(window.innerHeight * 0.6);
      if ("vibrate" in navigator) {
        navigator.vibrate(50);
      }
    }
  }, [bottomSheetHeight]);

  // Swipe gesture detection for collapsed state
  const [collapsedSwipeStartY, setCollapsedSwipeStartY] = useState<
    number | null
  >(null);
  const [collapsedSwipeStartTime, setCollapsedSwipeStartTime] = useState<
    number | null
  >(null);

  const handleCollapsedTouchStart = useCallback((e: React.TouchEvent) => {
    setCollapsedSwipeStartY(e.touches[0].clientY);
    setCollapsedSwipeStartTime(Date.now());
  }, []);

  const handleCollapsedTouchMove = useCallback(
    (e: React.TouchEvent) => {
      if (collapsedSwipeStartY === null) return;

      const currentY = e.touches[0].clientY;
      const deltaY = collapsedSwipeStartY - currentY;

      // If swiping up more than 50px, expand the panel
      if (deltaY > 50) {
        handleSwipeUp();
        setCollapsedSwipeStartY(null);
        setCollapsedSwipeStartTime(null);
      }
    },
    [collapsedSwipeStartY, handleSwipeUp]
  );

  const handleCollapsedTouchEnd = useCallback(
    (e: React.TouchEvent) => {
      if (collapsedSwipeStartY === null || collapsedSwipeStartTime === null)
        return;

      const currentY = e.changedTouches[0].clientY;
      const deltaY = collapsedSwipeStartY - currentY;
      const deltaTime = Date.now() - collapsedSwipeStartTime;
      const velocity = deltaY / deltaTime;

      // If swiping up with sufficient velocity or distance, expand
      if (deltaY > 30 || velocity > 0.5) {
        handleSwipeUp();
      }

      setCollapsedSwipeStartY(null);
      setCollapsedSwipeStartTime(null);
    },
    [collapsedSwipeStartY, collapsedSwipeStartTime, handleSwipeUp]
  );

  // Function to reopen bottom sheet
  const reopenBottomSheet = () => {
    setIsBottomSheetVisible(true);
    setBottomSheetHeight(120);
  };

  return (
    <div className="h-screen flex bg-background overflow-hidden">
      {/* Left Sidebar */}
      {isControlPanelVisible && (
        <div className="hidden lg:flex flex-col w-[460px] bg-background border-r border-border shadow-xl">
          {/* Sidebar Header with App Branding */}
          <div className="p-6 border-b border-border">
            <div className="flex items-center gap-3">
              <img
                src="/Futuricty1.png"
                alt="Futuricity Logo"
                className="w-24 h-15"
              />
              <div>
                <h1 className="text-xl font-bold bg-primary bg-clip-text text-transparent">
                  Futuricty
                </h1>
                <p className="text-xs text-muted-foreground">
                  Building Tomorrow
                </p>
              </div>
            </div>

            {/* Mode, Language, Theme Toggles */}
            <div className="flex items-center gap-2 mt-4">
              <LanguageToggle />
              <ThemeToggle />
            </div>
          </div>

          {/* Location List */}
          <div className="p-4 border-b border-border bg-muted/30">
            <h3 className="text-sm font-semibold mb-3">
              Selected Locations ({selectedLocations.length}/3)
            </h3>
            <div className="space-y-2">
              {selectedLocations.map((loc, idx) => (
                <div
                  key={idx}
                  className={`flex items-center justify-between p-2 rounded-md ${
                    activeLocationIndex === idx && hasCalculated
                      ? "bg-primary/10 border border-primary/20"
                      : "bg-background border border-border"
                  }`}
                >
                  <div className="flex items-center gap-2 overflow-hidden">
                    <div className="flex-shrink-0 w-5 h-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center text-xs font-bold">
                      {idx + 1}
                    </div>
                    <span className="text-sm truncate">
                      {loc.address ||
                        `${loc.lat.toFixed(4)}, ${loc.lng.toFixed(4)}`}
                    </span>
                  </div>
                  {!hasCalculated && (
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-6 w-6"
                      onClick={() => removeLocation(idx)}
                    >
                      <X className="w-3 h-3" />
                    </Button>
                  )}
                  {hasCalculated && (
                    <Button
                      variant={
                        activeLocationIndex === idx ? "default" : "outline"
                      }
                      size="sm"
                      className="h-7 text-xs"
                      onClick={() => setActiveLocationIndex(idx)}
                    >
                      View
                    </Button>
                  )}
                </div>
              ))}
              {selectedLocations.length === 0 && (
                <div className="text-sm text-muted-foreground text-center py-4">
                  Select up to 3 locations on the map
                </div>
              )}
            </div>

            {!hasCalculated && (
              <Button
                className="w-full mt-4"
                onClick={handleAnalyzeLocation}
                disabled={selectedLocations.length !== 3 || isCalculating}
              >
                {isCalculating ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Analyzing...
                  </>
                ) : selectedLocations.length === 0 ? (
                  "Select 3 locations"
                ) : selectedLocations.length < 3 ? (
                  `Select ${3 - selectedLocations.length} more location${
                    3 - selectedLocations.length !== 1 ? "s" : ""
                  }`
                ) : (
                  "Analyze 3 Locations"
                )}
              </Button>
            )}
          </div>

          {/* Sidebar Content */}
          <div className="flex-1 overflow-y-auto control-panel-scrollbar p-6 space-y-6">
            {hasCalculated ? (
              <>
                <LiveabilityScore data={livabilityData} />
                <ControlPanel
                  showRadius={showRadius}
                  onToggleRadius={() => setShowRadius(!showRadius)}
                  radiusOptions={radiusOptions}
                  isCalculating={isCalculating}
                  selectedLocations={selectedLocations}
                  activeLocationIndex={activeLocationIndex}
                  onRecalculate={handleRecalculate}
                  onAnalyzeLocation={handleRecalculate}
                  hasCalculated={hasCalculated}
                  livabilityData={livabilityData}
                  facilities={facilities}
                  className="w-full"
                />{" "}
              </>
            ) : (
              <div className="flex flex-col items-center justify-center h-full text-muted-foreground opacity-50">
                <MapPin className="w-12 h-12 mb-2" />
                <p>Select locations and click Analyze</p>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Full Screen Map */}
      <div className="flex-1 relative">
        <Map
          onLocationSelect={(lng, lat, address) =>
            handleLocationSelect(lng, lat, address)
          }
          selectedLocations={selectedLocations}
          activeLocationIndex={activeLocationIndex}
          facilities={facilities}
          showRadius={showRadius && hasCalculated && facilities.length > 0}
          radiusOptions={radiusOptions}
          hasCalculated={hasCalculated}
          visibleCategories={visibleCategories}
        />

        {/* Top Left Controls */}
        <div className="absolute top-4 left-4 z-10 flex items-center gap-3">
          <Button
            variant="outline"
            size="sm"
            onClick={toggleControlPanel}
            className="hidden lg:flex items-center justify-center w-10 h-10 bg-background/90 hover:bg-background border-border shadow-lg backdrop-blur-sm rounded-full"
            title={
              isControlPanelVisible
                ? t("hide.control.panel")
                : t("show.control.panel")
            }
          >
            <Menu className="w-4 h-4" />
          </Button>

          {/* Search Bar */}
          <div className="w-80">
            <LocationSearch
              onLocationSelect={(lng, lat, address) =>
                handleLocationSelect(lng, lat, address)
              }
            />
          </div>
        </div>
      </div>

      {/* Mobile Bottom Sheet - Google Maps Style */}
      {isBottomSheetVisible && (
        <div
          className={`fixed bottom-0 left-0 right-0 z-50 lg:hidden mobile-bottom-sheet mobile-bottom-sheet-snap ${
            bottomSheetHeight > 200
              ? "bg-background/95 backdrop-blur-md"
              : "bg-background/80 backdrop-blur-sm"
          } ${isDragging ? "mobile-bottom-sheet-dragging" : ""}`}
          style={{
            height: `${bottomSheetHeight}px`,
            transform: `translateY(${bottomSheetHeight === 0 ? "100%" : "0"})`,
            transition: isDragging
              ? "none"
              : "transform 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94), height 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94)",
          }}
        >
          {/* Drag Handle */}
          <div
            className="flex justify-center pt-3 pb-4 mobile-bottom-sheet-handle"
            onTouchStart={handleBottomSheetTouchStart}
            onTouchMove={handleBottomSheetTouchMove}
            onTouchEnd={handleBottomSheetTouchEnd}
          >
            <div className="w-16 h-1.5 bg-muted-foreground/60 rounded-full"></div>
          </div>

          {/* Content */}
          <div
            className="overflow-hidden"
            style={{ height: `calc(${bottomSheetHeight}px - 60px)` }}
          >
            {bottomSheetHeight > 200 ? (
              <div
                className="h-full overflow-y-auto mobile-scrollbar mobile-bottom-sheet-content expanded"
                style={{
                  height: "100%",
                  minHeight: "0",
                  maxHeight: "none",
                }}
              >
                <div className="p-6 space-y-6">
                  {/* Mobile Header */}
                  <div className="flex items-center justify-between pb-6 border-b border-[hsl(var(--control-border))]/30">
                    <div className="flex items-center gap-3">
                      <img
                        src="/futuricity-logo.svg"
                        alt="Futuricity Logo"
                        className="w-10 h-10"
                      />
                      <div>
                        <h2 className="text-lg font-bold text-[hsl(var(--control-primary))]">
                          Futuricity
                        </h2>
                        <p className="text-xs text-[hsl(var(--control-primary))]/70">
                          Building Tomorrow
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      <LanguageToggle />
                      <ThemeToggle />
                    </div>
                  </div>

                  {/* Mobile Location Selector */}
                  {hasCalculated && selectedLocations.length > 1 && (
                    <div className="flex gap-2 overflow-x-auto pb-2">
                      {selectedLocations.map((_, idx) => (
                        <Button
                          key={idx}
                          variant={
                            activeLocationIndex === idx ? "default" : "outline"
                          }
                          size="sm"
                          onClick={() => setActiveLocationIndex(idx)}
                        >
                          Loc {idx + 1}
                        </Button>
                      ))}
                    </div>
                  )}

                  {hasCalculated && livabilityData ? (
                    <LiveabilityScore data={livabilityData} />
                  ) : null}

                  <ControlPanel
                    showRadius={showRadius}
                    onToggleRadius={handleToggleRadius}
                    radiusOptions={radiusOptions}
                    isCalculating={isCalculating}
                    selectedLocations={selectedLocations}
                    activeLocationIndex={activeLocationIndex}
                    onRecalculate={handleRecalculate}
                    onAnalyzeLocation={handleRecalculate}
                    hasCalculated={hasCalculated}
                    livabilityData={livabilityData}
                    facilities={facilities}
                  />

                  {!hasCalculated && (
                    <div className="space-y-4">
                      <h3 className="font-semibold">Selected Locations</h3>
                      {selectedLocations.map((loc, idx) => (
                        <div
                          key={idx}
                          className="flex justify-between items-center bg-muted p-2 rounded"
                        >
                          <span className="text-sm truncate w-48">
                            {loc.address || "Location " + (idx + 1)}
                          </span>
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={() => removeLocation(idx)}
                          >
                            <X className="w-4 h-4" />
                          </Button>
                        </div>
                      ))}
                      <Button
                        className="w-full"
                        onClick={handleAnalyzeLocation}
                        disabled={
                          selectedLocations.length !== 3 || isCalculating
                        }
                      >
                        {isCalculating ? (
                          <>
                            <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                            Analyzing...
                          </>
                        ) : selectedLocations.length === 3 ? (
                          "Analyze 3 Locations"
                        ) : (
                          `Select ${
                            3 - selectedLocations.length
                          } more location${
                            3 - selectedLocations.length !== 1 ? "s" : ""
                          }`
                        )}
                      </Button>
                    </div>
                  )}
                </div>
              </div>
            ) : (
              <div
                className="p-6 mobile-bottom-sheet-content collapsed"
                onTouchStart={handleCollapsedTouchStart}
                onTouchMove={handleCollapsedTouchMove}
                onTouchEnd={handleCollapsedTouchEnd}
              >
                {/* Collapsed view content */}
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-[hsl(var(--control-bg))] rounded-lg">
                      <div className="w-4 h-4 bg-[hsl(var(--control-primary))] rounded-full"></div>
                    </div>
                    <h3 className="font-bold text-base text-[hsl(var(--control-primary))]">
                      {t("livability.score")}
                    </h3>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] text-[hsl(var(--control-primary))]/70 font-medium">
                      {t("swipe.up.details")}
                    </span>
                    <div className="w-5 h-5 text-[hsl(var(--control-primary))] mobile-swipe-indicator animate-bounce">
                      <svg
                        viewBox="0 0 24px 24px"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                      >
                        <path d="M7 14l5-5 5 5" />
                      </svg>
                    </div>
                  </div>
                </div>
                {hasCalculated && (
                  <div className="flex items-center gap-4">
                    <div className="text-2xl font-bold text-primary">
                      {Math.round(livabilityData.overall)}
                    </div>
                    <div className="flex-1">
                      <div className="w-full bg-muted rounded-full h-3 mb-2">
                        <div
                          className="h-3 rounded-full transition-all duration-1000 ease-out bg-primary"
                          style={{
                            width: `${livabilityData.overall}%`,
                          }}
                        />
                      </div>
                      <p className="text-xs text-foreground font-medium">
                        {selectedLocations[activeLocationIndex]?.address?.split(
                          ","
                        )[0] || t("selected.location")}
                      </p>
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      {/* Floating Button to Reopen Bottom Sheet */}
      {!isBottomSheetVisible && (
        <div className="fixed bottom-4 right-4 z-50 lg:hidden">
          <button
            onClick={reopenBottomSheet}
            className="w-14 h-14 bg-primary border border-primary rounded-full shadow-lg hover:shadow-xl transition-all duration-200 flex items-center justify-center"
            title="Open Control Panel"
          >
            <BarChart3 className="w-6 h-6 text-white" />
          </button>
        </div>
      )}

      {/* Cache Manager for Development */}
    </div>
  );
};

export default Index;
