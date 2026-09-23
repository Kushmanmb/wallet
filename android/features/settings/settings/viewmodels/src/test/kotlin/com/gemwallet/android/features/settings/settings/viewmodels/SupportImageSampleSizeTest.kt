package com.gemwallet.android.features.settings.settings.viewmodels

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SupportImageSampleSizeTest {

    @Test
    fun anImageWithinTheLimitIsDecodedAtFullSize() {
        assertEquals(1, supportImageSampleSize(2048, 1536, 2048))
        assertEquals(1, supportImageSampleSize(640, 480, 2048))
    }

    @Test
    fun aLargePhotoIsSampledDownUntilItsLongestSideFits() {
        val sampleSize = supportImageSampleSize(12_000, 9_000, 2048)

        assertEquals(8, sampleSize)
        assertTrue(12_000 / sampleSize <= 2048)
        assertTrue(9_000 / sampleSize > 0)
    }
}
