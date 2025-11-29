import React, { useState, useCallback } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import {
  Eye,
  EyeOff,
  Target,
  Loader2,
  MapPin,
} from "lucide-react";
import { useLanguage } from "./LanguageProvider";
import { useIsMobile } from "@/hooks/use-mobile";

interface ControlPanelProps {
  showRadius: boolean;
  onToggleRadius: () => void;
  radiusOptions: number[];
  isCalculating: boolean;
  selectedLocations: Array<{ lng: number; lat: number; address?: string }>;
  activeLocationIndex?: number;
  onRecalculate: () => void;
  onAnalyzeLocation: () => void;
  hasCalculated: boolean;
  livabilityData?: {
    overall: number;
    subscores: {
      services: number;
      mobility: number;
      safety: number;
      environment: number;
      [key: string]: number;
    };
    facilityCounts: {
      health: number;
      education: number;
      market: number;
      transport: number;
      walkability: number;
      recreation: number;
      safety: number;
      accessibility: number;
      police: number;
      religious: number;
      [key: string]: number;
    };
  };
  facilities?: Array<{
    name: string;
    category: string;
    distance: number;
  }>;
  className?: string;
}

const ControlPanel: React.FC<ControlPanelProps> = ({
  showRadius,
  onToggleRadius,
  radiusOptions,
  isCalculating,
  selectedLocations,
  activeLocationIndex = 0,
  onRecalculate,
  onAnalyzeLocation,
  hasCalculated,
  livabilityData,
  facilities,
  className,
}) => {
  const { t } = useLanguage();
  const isMobile = useIsMobile();
  
  const activeLocation = selectedLocations[activeLocationIndex];

  return (
    <Card
      className={`bg-gradient-to-br from-background via-background/95 to-accent/5 border-border/50 shadow-lg backdrop-blur-sm ${className}`}
    >
      <CardContent className="p-6 space-y-6">
        {/* Current Location Section */}
        <div className="space-y-3">
          <div className="flex items-center gap-3 pb-2 border-b border-[hsl(var(--control-border))]">
            <div className="p-2 bg-[hsl(var(--control-bg-light))] rounded-lg">
              <MapPin className="w-5 h-5 text-[hsl(var(--control-primary))]" />
            </div>
            <div>
              <h3 className="text-sm font-semibold text-[hsl(var(--control-primary))]">
                {t("current.location")}
              </h3>
              <p className="text-xs text-[hsl(var(--control-primary))]/70">
                {t("selected.area.analysis")}
              </p>
            </div>
          </div>

          {activeLocation ? (
            <div className="space-y-3">
              <div className="p-4 bg-gradient-to-r from-[hsl(var(--control-bg-light))] to-[hsl(var(--control-bg))] rounded-xl border border-[hsl(var(--control-border))] control-panel-box">
                <div className="flex items-start gap-3">
                  <MapPin className="w-5 h-5 mt-0.5 text-[hsl(var(--control-primary))] flex-shrink-0" />
                  <div className="min-w-0 flex-1">
                    <p className="text-sm font-medium text-[hsl(var(--control-primary))] break-words leading-tight">
                      {activeLocation.address || t("loading.address")}
                    </p>
                    <p className="text-xs text-[hsl(var(--control-primary))]/70 mt-1 font-mono">
                      {activeLocation.lat.toFixed(6)},{" "}
                      {activeLocation.lng.toFixed(6)}
                    </p>
                  </div>
                </div>
              </div>

              {!hasCalculated ? (
                <Button
                  onClick={onAnalyzeLocation}
                  disabled={isCalculating || selectedLocations.length !== 3}
                  size="sm"
                  className="w-full h-11 bg-gradient-to-r from-primary to-primary/80 hover:from-primary/90 hover:to-primary/70 text-white shadow-xl hover:shadow-2xl transition-all duration-200"
                >
                  {isCalculating ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      {t("analyzing.location")}
                    </>
                  ) : (
                    <>
                      <Target className="w-4 h-4 mr-2" />
                      {selectedLocations.length === 3 
                        ? t("analyze.3.locations") 
                        : t("select.3.locations", { count: 3 - selectedLocations.length })}
                    </>
                  )}
                </Button>
              ) : (
                <Button
                  onClick={onRecalculate}
                  disabled={isCalculating}
                  size="sm"
                  variant="outline"
                  className="w-full h-11 border-2 border-[hsl(var(--control-primary))] hover:bg-[hsl(var(--control-primary))]/10 text-[hsl(var(--control-primary))] transition-all duration-200"
                >
                  {isCalculating ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      {t("recalculating")}
                    </>
                  ) : (
                    <>
                      <Target className="w-4 h-4 mr-2" />
                      {t("recalculate.score")}
                    </>
                  )}
                </Button>
              )}

            </div>
          ) : (
            <div className="p-4 bg-[hsl(var(--control-bg-light))] rounded-xl border border-[hsl(var(--control-border))] text-center control-panel-box">
              <MapPin className="w-10 h-10 mx-auto text-[hsl(var(--control-primary))] mb-2" />
              <p className="text-sm text-[hsl(var(--control-primary))]">
                {t("no.location.selected")}
              </p>
              <p className="text-xs text-[hsl(var(--control-primary))]/70 mt-1">
                {t("click.map.choose.residents")}
              </p>
            </div>
          )}
        </div>

        {/* Facility Counts Section */}
        {livabilityData && (
          <div className="space-y-3">
            <div className="flex items-center gap-3 pb-2 border-b border-[hsl(var(--control-border))]">
              <div className="p-2 bg-[hsl(var(--control-bg-light))] rounded-lg">
                <div className="w-5 h-5 bg-[hsl(var(--control-primary))] rounded-full"></div>
              </div>
              <div>
                <h3 className="text-sm font-semibold text-[hsl(var(--control-primary))]">
                  {t("facility.counts")}
                </h3>
                <p className="text-xs text-[hsl(var(--control-primary))]/70">
                  {t("found.in.selected.area")}
                </p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-red-500/10 to-pink-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("health.emoji")}
                </span>
                <span className="text-lg font-bold text-red-500">
                  {livabilityData.facilityCounts.health}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-blue-500/10 to-indigo-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("education.emoji")}
                </span>
                <span className="text-lg font-bold text-blue-500">
                  {livabilityData.facilityCounts.education}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-amber-500/10 to-orange-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("markets.emoji")}
                </span>
                <span className="text-lg font-bold text-amber-500">
                  {livabilityData.facilityCounts.market}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-purple-500/10 to-violet-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("transport.emoji")}
                </span>
                <span className="text-lg font-bold text-purple-500">
                  {livabilityData.facilityCounts.transport}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-orange-500/10 to-red-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("walkability.emoji")}
                </span>
                <span className="text-lg font-bold text-orange-500">
                  {livabilityData.facilityCounts.walkability}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-teal-500/10 to-cyan-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("recreation.emoji")}
                </span>
                <span className="text-lg font-bold text-teal-500">
                  {livabilityData.facilityCounts.recreation}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-green-500/10 to-emerald-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("safety.emoji")}
                </span>
                <span className="text-lg font-bold text-green-500">
                  {livabilityData.facilityCounts.safety}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-indigo-500/10 to-blue-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("police.emoji")}
                </span>
                <span className="text-lg font-bold text-indigo-500">
                  {livabilityData.facilityCounts.police}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-gray-500/10 to-slate-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("religious.emoji")}
                </span>
                <span className="text-lg font-bold text-gray-500">
                  {livabilityData.facilityCounts.religious}
                </span>
              </div>
              <div className="flex justify-between items-center p-3 bg-gradient-to-r from-pink-500/10 to-rose-500/10 rounded-xl border border-border/30">
                <span className="text-sm text-foreground font-medium">
                  {t("accessibility.emoji")}
                </span>
                <span className="text-lg font-bold text-pink-500">
                  {livabilityData.facilityCounts.accessibility}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Nearby Facilities Section */}
        {facilities &&
          facilities.length > 0 &&
          (() => {
            // Filter facilities: within 500m, exclude safety and walkability
            const filteredFacilities = facilities
              .filter(
                (facility) =>
                  facility.distance <= 500 &&
                  facility.category !== "safety" &&
                  facility.category !== "walkability"
              )
              .sort((a, b) => a.distance - b.distance); // Sort by distance (nearest first)

            return (
              <div className="space-y-3">
                <div className="flex items-center gap-3 pb-2 border-b border-[hsl(var(--control-border))]">
                  <div className="p-2 bg-[hsl(var(--control-bg-light))] rounded-lg">
                    <div className="w-5 h-5 bg-[hsl(var(--control-primary))] rounded-full"></div>
                  </div>
                  <div>
                    <h3 className="text-sm font-semibold text-[hsl(var(--control-primary))]">
                      {t("nearby.facilities")}
                    </h3>
                    <p className="text-xs text-[hsl(var(--control-primary))]/70">
                      {t("nearby.facilities.description")}
                    </p>
                  </div>
                </div>

                {filteredFacilities.length > 0 ? (
                  <div className="space-y-3 max-h-60 overflow-y-auto">
                    {filteredFacilities.map((facility, index) => (
                      <div
                        key={index}
                        className="flex justify-between items-center p-3 bg-gradient-to-r from-accent/20 to-accent/10 rounded-xl border border-border/30 hover:bg-accent/30 transition-all duration-200"
                      >
                        <div className="flex-1 min-w-0">
                          <p className="text-sm font-medium text-foreground truncate">
                            {facility.name}
                          </p>
                          <p className="text-xs text-muted-foreground capitalize">
                            {facility.category}
                          </p>
                        </div>
                        <span className="text-sm bg-background/80 backdrop-blur-sm border border-border/50 px-3 py-1.5 rounded-full ml-3 font-medium text-foreground">
                          {facility.distance}
                          {t("meters")}
                        </span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="p-4 text-center text-sm text-muted-foreground">
                    {t("no.facilities.found")}
                  </div>
                )}
              </div>
            );
          })()}

        {/* Map Control Section */}
        <div className="space-y-3">
          <div className="flex items-center gap-3 pb-2 border-b border-[hsl(var(--control-border))]">
            <div className="p-2 bg-[hsl(var(--control-bg-light))] rounded-lg">
              {showRadius ? (
                <Eye className="w-5 h-5 text-[hsl(var(--control-primary))]" />
              ) : (
                <EyeOff className="w-5 h-5 text-[hsl(var(--control-primary))]" />
              )}
            </div>
            <div className="flex-1">
              <h3 className="text-sm font-semibold text-[hsl(var(--control-primary))]">
                {t("map.controls")}
              </h3>
              <p className="text-xs text-[hsl(var(--control-primary))]/70">
                {t("map.controls.description")}
              </p>
            </div>
            <Switch
              id="radius-toggle"
              checked={showRadius}
              onCheckedChange={onToggleRadius}
              disabled={!hasCalculated}
              className="facility-switch"
            />
          </div>

          <div className="p-3 bg-gradient-to-r from-[hsl(var(--control-bg-light))] to-[hsl(var(--control-bg))] rounded-xl border border-[hsl(var(--control-border))] control-panel-box-medium">
            <p className="text-xs text-[hsl(var(--control-primary))]/70 text-center">
              {t("available.radius.options")}{" "}
              {radiusOptions.join(t("meters") + ", ")}
              {t("meters")}
            </p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default ControlPanel;
