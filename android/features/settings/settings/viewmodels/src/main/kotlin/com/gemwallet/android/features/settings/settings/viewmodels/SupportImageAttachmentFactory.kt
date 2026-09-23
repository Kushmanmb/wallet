package com.gemwallet.android.features.settings.settings.viewmodels

import android.content.Context
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.net.Uri
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.ByteArrayOutputStream
import javax.inject.Inject
import javax.inject.Singleton

private const val SUPPORT_IMAGE_JPEG_QUALITY = 90
private const val SUPPORT_IMAGE_MAX_DIMENSION = 2048

@Singleton
class SupportImageAttachmentFactory @Inject constructor(@param:ApplicationContext private val context: Context) {
    suspend fun fromUri(uri: Uri): ByteArray? = withContext(Dispatchers.IO) {
        val bounds = BitmapFactory.Options().also { options ->
            options.inJustDecodeBounds = true
            context.contentResolver.openInputStream(uri)?.use { BitmapFactory.decodeStream(it, null, options) } ?: return@withContext null
        }
        if (bounds.outWidth <= 0 || bounds.outHeight <= 0) return@withContext null
        val options = BitmapFactory.Options().also { it.inSampleSize = supportImageSampleSize(bounds.outWidth, bounds.outHeight, SUPPORT_IMAGE_MAX_DIMENSION) }
        val bitmap = context.contentResolver.openInputStream(uri)?.use { BitmapFactory.decodeStream(it, null, options) } ?: return@withContext null
        try {
            ByteArrayOutputStream().use { stream ->
                bitmap.compress(Bitmap.CompressFormat.JPEG, SUPPORT_IMAGE_JPEG_QUALITY, stream)
                stream.toByteArray()
            }
        } finally {
            bitmap.recycle()
        }
    }
}

internal fun supportImageSampleSize(width: Int, height: Int, maxDimension: Int): Int {
    var sampleSize = 1
    while (maxOf(width, height) / sampleSize > maxDimension) {
        sampleSize *= 2
    }
    return sampleSize
}
