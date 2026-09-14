// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import Formatters
import struct Gemstone.GemFormattedNumber
import enum Gemstone.GemNumberDisplay
import enum Gemstone.GemNumberUnit
import enum Gemstone.GemPrecision

public extension GemFormattedNumber {
    func text(locale: Locale = .current) -> String {
        switch display {
        case let .number(precision):
            appendingSymbol(numberText(precision: precision, locale: locale))
        case .abbreviated:
            abbreviatedText(locale: locale)
        case let .belowThreshold(threshold, places):
            appendingSymbol("<\(thresholdText(threshold, places: places, locale: locale))")
        }
    }
}

// MARK: - Private

private extension GemFormattedNumber {
    var currencyCode: String? {
        switch unit {
        case let .currency(code): code
        case .percent, .symbol, .plain: nil
        }
    }

    var symbol: String? {
        switch unit {
        case let .symbol(symbol): symbol
        case .currency, .percent, .plain: nil
        }
    }

    var percentSign: Bool? {
        switch unit {
        case let .percent(showsSign): showsSign
        case .currency, .symbol, .plain: nil
        }
    }

    func numberText(precision: GemPrecision, locale: Locale) -> String {
        if let percentSign {
            return value.formatted(
                .percent.locale(locale)
                    .precision(precision.formatStyle)
                    .sign(strategy: percentSign ? .always(includingZero: false) : .never)
                    .scale(1),
            )
        }
        guard let currencyCode else {
            return value.formatted(.number.locale(locale).precision(precision.formatStyle))
        }
        return value.formatted(.currency(code: currencyCode).locale(locale).precision(precision.formatStyle))
    }

    func abbreviatedText(locale: Locale) -> String {
        let formatter = AbbreviatedFormatter(locale: locale)
        if let currencyCode {
            return formatter.string(from: value, currency: currencyCode) ?? numberText(precision: .fraction(min: 2, max: 2), locale: locale)
        }
        return appendingSymbol(formatter.string(from: value) ?? numberText(precision: .fraction(min: 2, max: 2), locale: locale))
    }

    func thresholdText(_ threshold: Double, places: UInt32, locale: Locale) -> String {
        guard let currencyCode else {
            return threshold.formatted(.number.locale(locale).precision(.fractionLength(Int(places))))
        }
        return threshold.formatted(.currency(code: currencyCode).locale(locale).precision(.fractionLength(Int(places))))
    }

    func appendingSymbol(_ text: String) -> String {
        guard let symbol else { return text }
        return "\(text) \(symbol)"
    }
}
