package com.gemwallet.android.data.service.store.database.di

import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase

object Migration_96_97 : Migration(96, 97) {
    override fun migrate(db: SupportSQLiteDatabase) {
        for (column in listOf("marketCapUsd", "marketCapFdvUsd", "totalVolumeUsd", "allTimeHighUsd", "allTimeLowUsd")) {
            db.execSQL("ALTER TABLE `asset_market` ADD COLUMN `$column` REAL DEFAULT NULL")
        }
    }
}
