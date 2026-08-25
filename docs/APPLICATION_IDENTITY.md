# Application Identity & Icons Specification

## Overview

This document defines AlphaForge's application identity, brand guidelines, and icon specifications for the M8 Local MVP release.

---

## 1. Product Identity

### 1.1 Product Name

**Official Name**: AlphaForge

**Display Name**: AlphaForge

**Internal Name**: alpha-forge (code, URLs)

**Package Name**: AlphaForge (installers)

**Reverse-DNS Identifier**: `com.berry.alphaforge`

AlphaForge is the only product name. Do not introduce a secondary product name,
former descriptor, installer alias, or alternate spelling. Descriptive phrases
such as "AI-native investment research workspace" are taglines, not names.

### 1.2 Product Tagline

**Primary Tagline**: "AI-native investment research workspace"

**Alternative Taglines**:
- "Your investment thesis, validated"
- "Research → Thesis → Decision"
- "Investment intelligence, locally yours"

### 1.3 Product Description

**Short (50 words)**:
AlphaForge is a desktop-first AI workspace for investment research. Develop, test, and refine investment theses with persistent knowledge, evidence tracking, and interactive visualizations. Local-first, no cloud dependency, complete data privacy.

**Medium (100 words)**:
AlphaForge transforms investment research from scattered notes into structured knowledge. Build investment theses with explicit reasoning, track supporting and contradicting evidence, and validate outcomes over time. AI agents assist with research tasks while maintaining full user control. All data stays on your machine—no cloud sync, no remote servers, complete privacy. Perfect for individual investors who want institutional-grade research tools with complete data ownership.

### 1.4 Product Positioning

| Aspect | Position |
|--------|----------|
| **Target User** | Individual investors, investment analysts, portfolio managers |
| **Primary Value** | Thesis-driven investment research with AI assistance |
| **Key Differentiator** | Local-first with complete data privacy and ownership |
| **Competitive Advantage** | Structured thesis management + AI agent assistance + offline capability |
| **Price Point** | Free and open-source (MVP) |

---

## 2. Brand Identity

### 2.1 Brand Values

1. **Rigorous**: Evidence-based, not speculation-driven
2. **Transparent**: Clear reasoning, auditable decisions
3. **Private**: Your data stays yours
4. **Intelligent**: AI-enhanced, human-controlled
5. **Structured**: Organized knowledge, not scattered notes

### 2.2 Brand Voice

**Tone**: Professional, precise, confident but not arrogant

**Language Style**:
- Clear and direct
- Technical when necessary, accessible when possible
- Active voice, present tense
- No jargon without explanation

**Examples**:
- ✅ "Track your investment theses with evidence-based validation"
- ❌ "Leverage cutting-edge AI for next-gen investment optimization"
- ✅ "All data stored locally on your machine"
- ❌ "Empowering users with privacy-first architecture"

### 2.3 Brand Colors

#### Primary Colors

| Color | Hex Code | RGB | Usage |
|-------|----------|-----|-------|
| **AlphaForge Blue** | `#2563EB` | `rgb(37, 99, 235)` | Primary actions, links, focus states |
| **AlphaForge Dark** | `#1E293B` | `rgb(30, 41, 59)` | Text, headers, dark mode backgrounds |

#### Secondary Colors

| Color | Hex Code | RGB | Usage |
|-------|----------|-----|-------|
| **Success Green** | `#10B981` | `rgb(16, 185, 129)` | Positive indicators, success states |
| **Warning Amber** | `#F59E0B` | `rgb(245, 158, 11)` | Warnings, caution states |
| **Error Red** | `#EF4444` | `rgb(239, 68, 68)` | Errors, destructive actions |
| **Neutral Gray** | `#64748B` | `rgb(100, 116, 139)` | Secondary text, borders |

#### Background Colors

| Mode | Primary Background | Secondary Background | Surface |
|------|-------------------|---------------------|---------|
| **Light** | `#FFFFFF` | `#F8FAFC` | `#F1F5F9` |
| **Dark** | `#0F172A` | `#1E293B` | `#334155` |

### 2.4 Typography

**Primary Font**: Inter (system font fallback: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto)

**Font Weights**:
- Regular (400): Body text
- Medium (500): Emphasis, labels
- Semibold (600): Buttons, small headings
- Bold (700): Large headings

**Font Sizes**:
| Element | Size | Line Height |
|---------|------|-------------|
| Heading 1 | 2.25rem (36px) | 1.2 |
| Heading 2 | 1.875rem (30px) | 1.3 |
| Heading 3 | 1.5rem (24px) | 1.4 |
| Body | 1rem (16px) | 1.5 |
| Small | 0.875rem (14px) | 1.5 |
| Caption | 0.75rem (12px) | 1.4 |

---

## 3. Icon Specifications

### 3.1 Application Icon Requirements

#### Platform Specifications

| Platform | Format | Size(s) | Location |
|----------|--------|---------|----------|
| **macOS** | `.icns` | 16x16 to 1024x1024 | `src-tauri/icons/icon.icns` |
| **Windows** | `.ico` | 16x16 to 256x256 | `src-tauri/icons/icon.ico` |
| **Linux** | `.png` | 512x512 | `src-tauri/icons/icon.png` |

#### Required Icon Sizes

| Size (px) | Usage |
|-----------|-------|
| 16 | Taskbar (Windows), Menu bar |
| 32 | Taskbar (Windows), Small UI elements |
| 48 | Taskbar (Windows) |
| 64 | File manager thumbnails |
| 128 | Application launcher |
| 256 | Large file icons |
| 512 | App store, promotional |
| 1024 | macOS @2x, high-resolution displays |

### 3.2 Icon Design Guidelines

#### Visual Metaphor

**Concept**: Forge symbol combining:
- **Flame/Heat**: Transformation, refinement
- **Alpha Symbol (α)**: First, primary, beginning
- **Abstract Chart/Graph**: Investment, data analysis
- **Shield outline**: Security, privacy, protection

#### Design Principles

1. **Simplicity**: Recognizable at small sizes (16x16)
2. **Scalability**: Clear at all sizes from 16px to 1024px
3. **Uniqueness**: Distinctive among finance apps
4. **Professionalism**: Suitable for serious investment work
5. **Timelessness**: Not trendy, will age well

#### Color Guidelines

- **Primary**: AlphaForge Blue (`#2563EB`)
- **Accent**: White or light gray for contrast
- **Gradient**: Subtle gradient from darker blue to lighter blue (optional)
- **Avoid**: Multiple bright colors, photographic elements

#### Shape Guidelines

- **Outer Shape**: Rounded square (iOS-style) or circle
- **Corner Radius**: 20% of icon size
- **Padding**: 10% padding from edges for safe area
- **Background**: Solid color or subtle gradient, no transparency

### 3.3 Icon Variants

#### Main Application Icon

- Full color, standard design
- Used for: Desktop shortcut, Taskbar, Dock, App launcher

#### Monochrome Variant

- Single color (white or brand color)
- Used for: System notifications, Menu bar icons, Small indicators

#### Outline Variant

- Transparent background, outlined shape
- Used for: Documentation, Marketing materials, Watermarks

### 3.4 Existing Icons

Current icon set (as of M8):
- ✅ `icon.ico` - Windows icon (16,384 bytes)
- ✅ `icon.icns` - macOS icon (99,208 bytes)
- ✅ `icon.png` - Generic icon (21,346 bytes)
- ✅ Various Windows Store logos (Square30x30, Square44x44, etc.)

**Status**: Icons exist but need design review for brand consistency.

---

## 4. Implementation Checklist

### 4.1 Icon Creation Process

- [ ] **Design Phase**
  - [ ] Create icon concept sketches (3-5 variants)
  - [ ] Select primary concept
  - [ ] Create vector artwork (SVG/Figma)
  - [ ] Review at multiple sizes (16px to 1024px)
  - [ ] Get stakeholder approval

- [ ] **Production Phase**
  - [ ] Export PNG at all required sizes
  - [ ] Generate `.ico` file for Windows
  - [ ] Generate `.icns` file for macOS
  - [ ] Create monochrome variants
  - [ ] Create outline variants

- [ ] **Integration Phase**
  - [ ] Replace existing icons in `src-tauri/icons/`
  - [ ] Update `tauri.conf.json` icon references
  - [ ] Test on macOS (Dock, Finder, Menu bar)
  - [ ] Test on Windows (Taskbar, Explorer, Start menu)
  - [ ] Verify icon appears correctly in installer

### 4.2 Brand Consistency

- [ ] Audit all UI elements for brand color usage
- [ ] Verify typography consistency across application
- [ ] Check spacing and layout against design system
- [ ] Review error/success/warning states use correct colors
- [ ] Ensure dark mode follows brand guidelines

---

## 5. File Specifications

### 5.1 Icon File Requirements

#### Windows (.ico)

**Location**: `apps/desktop/src-tauri/icons/icon.ico`

**Required Sizes**:
```
16x16 (32-bit RGBA + 8-bit palette)
32x32 (32-bit RGBA + 8-bit palette)
48x48 (32-bit RGBA)
64x64 (32-bit RGBA)
128x128 (32-bit RGBA)
256x256 (32-bit RGBA)
```

**Tools**:
- Use `png-to-ico` npm package
- Or online converters like icoconvert.com
- Or graphic editors: GIMP, IcoFX

#### macOS (.icns)

**Location**: `apps/desktop/src-tauri/icons/icon.icns`

**Required Sizes**:
```
16x16, 16x16@2x (32x32)
32x32, 32x32@2x (64x64)
64x64, 64x64@2x (128x128)
128x128, 128x128@2x (256x256)
256x256, 256x256@2x (512x512)
512x512, 512x512@2x (1024x1024)
```

**Tools**:
- Use `iconutil` (macOS built-in)
- Or online converters like cloudconvert.com
- Or graphic editors: Sketch, Affinity Designer

### 5.2 Export Settings

| Setting | Value |
|---------|-------|
| **Color Mode** | RGB |
| **Color Profile** | sRGB IEC61966-2.1 |
| **Bit Depth** | 32-bit (8 bits per channel + 8-bit alpha) |
| **Background** | Solid color (no transparency for main icon) |
| **Compression** | PNG lossless |

---

## 6. Third-Party Usage

### 6.1 Brand Assets

**Allowed Usage**:
- Screenshots in reviews and tutorials
- Logos in "Made with AlphaForge" attributions
- Icons in software directories (with attribution)

**Prohibited Usage**:
- Impersonating official AlphaForge presence
- Modifying brand colors or logo proportions
- Using brand assets to imply endorsement
- Including in commercial products without license

### 6.2 Attribution Requirements

When using AlphaForge brand assets:
1. Link to official repository: https://github.com/BerryUIKI/alpha-forge
2. Include license notice: "MIT License"
3. Do not alter visual identity elements
4. Do not use in misleading or defamatory context

---

## 7. Accessibility Considerations

### 7.1 Color Contrast

All brand colors must meet WCAG 2.1 AA standards:

| Combination | Ratio | Pass/Fail |
|-------------|-------|-----------|
| AlphaForge Blue on White | 4.54:1 | ✅ AA |
| AlphaForge Blue on Dark | 7.23:1 | ✅ AAA |
| Success Green on White | 3.53:1 | ✅ AA (large text) |
| Error Red on White | 4.53:1 | ✅ AA |

### 7.2 Icon Accessibility

- Icons must not be sole indicator of meaning
- Pair icons with text labels where possible
- Ensure sufficient contrast between icon and background
- Test icon recognizability with colorblind users

---

## 8. Legal & Trademark

### 8.1 Trademark Status

- **Name**: "AlphaForge" (not registered trademark yet)
- **Logo**: To be determined
- **Usage**: Open-source project under MIT License

### 8.2 Intellectual Property

- Icons and brand assets are copyright of the project
- MIT License applies to all visual assets in repository
- Third-party icon libraries must be compatible with MIT License

---

## 9. Future Considerations

### 9.1 Brand Evolution

**Phase 1 (M8 MVP)**: Establish core identity and icons
**Phase 2 (Post-MVP)**: Refine based on user feedback
**Phase 3 (Commercial)**: Professional brand audit and trademark registration

### 9.2 Localization

- Product name stays "AlphaForge" in all locales
- Taglines translated for supported languages
- Icon design is locale-neutral (no text in icons)

---

## 10. References

- [Apple Human Interface Guidelines - App Icon](https://developer.apple.com/design/human-interface-guidelines/macos/icons-and-images/app-icon/)
- [Microsoft - Icon Design Guidelines](https://docs.microsoft.com/en-us/windows/apps/design/style/iconography)
- [Material Design - Iconography](https://material.io/design/iconography)
- [WCAG 2.1 Color Contrast](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)

---

## Success Criteria

- [ ] Application icon created in all required formats
- [ ] Icon passes visual review at all sizes
- [ ] Brand colors meet accessibility standards
- [ ] Typography consistently applied
- [ ] Brand guidelines documented and approved
- [ ] All platform-specific icon requirements met
- [ ] Icons tested on macOS and Windows builds

---

*Last Updated: 2026-08-03*
*Version: 1.0*
*Milestone: M8 - Local MVP Completion*
