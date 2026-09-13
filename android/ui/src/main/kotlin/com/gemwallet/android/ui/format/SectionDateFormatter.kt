package com.gemwallet.android.ui.format

import java.time.Clock
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.time.format.FormatStyle
import java.util.Locale
import uniffi.gemstone.GemDay
import uniffi.gemstone.GemDayBoundaries

class SectionDateFormatter(
    private val todayLabel: String,
    private val yesterdayLabel: String,
    private val boundaries: GemDayBoundaries = LocalDate.now().gemDay().boundaries(),
) {
    constructor(todayLabel: String, yesterdayLabel: String, clock: Clock) : this(
        todayLabel = todayLabel,
        yesterdayLabel = yesterdayLabel,
        boundaries = LocalDate.now(clock).gemDay().boundaries(),
    )

    fun format(date: LocalDate, locale: Locale): String = when (date.gemDay()) {
        boundaries.today -> todayLabel
        boundaries.yesterday -> yesterdayLabel
        else -> DateTimeFormatter
            .ofLocalizedDate(FormatStyle.LONG)
            .withLocale(locale)
            .format(date)
    }
}

internal fun LocalDate.gemDay(): GemDay = GemDay(year = year, month = monthValue.toUInt(), day = dayOfMonth.toUInt())
